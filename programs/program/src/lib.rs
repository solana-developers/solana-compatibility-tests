use anchor_lang::prelude::*;
use anchor_spl::token::ID as TokenProgramId;

declare_id!("2PHLJwx7Tp2L5CJPZfreVPZQnhgT9D6PwvMdGAn6pHRb");

#[program]
mod basic_0 {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("The Token Program ID is: {:?}", TokenProgramId);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}