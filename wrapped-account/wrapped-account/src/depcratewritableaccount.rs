// Generated macro for WritableAccount (trait)
macro_rules! DepcrateWritableAccount {
() => {
// Module: crate
// Provides: {"WritableAccount"}
// Dependencies: {}
pub trait WritableAccount : ReadableAccount { fn set_lamports (& mut self , lamports : u64) ; fn checked_add_lamports (& mut self , lamports : u64) -> Result < () , LamportsError > { self . set_lamports (self . lamports () . checked_add (lamports) . ok_or (LamportsError :: ArithmeticOverflow) ? ,) ; Ok (()) } fn checked_sub_lamports (& mut self , lamports : u64) -> Result < () , LamportsError > { self . set_lamports (self . lamports () . checked_sub (lamports) . ok_or (LamportsError :: ArithmeticUnderflow) ? ,) ; Ok (()) } fn saturating_add_lamports (& mut self , lamports : u64) { self . set_lamports (self . lamports () . saturating_add (lamports)) } fn saturating_sub_lamports (& mut self , lamports : u64) { self . set_lamports (self . lamports () . saturating_sub (lamports)) } fn data_as_mut_slice (& mut self) -> & mut [u8] ; fn set_owner (& mut self , owner : Pubkey) ; fn copy_into_owner_from_slice (& mut self , source : & [u8]) ; fn set_executable (& mut self , executable : bool) ; fn set_rent_epoch (& mut self , epoch : Epoch) ; fn create (lamports : u64 , data : Vec < u8 > , owner : Pubkey , executable : bool , rent_epoch : Epoch ,) -> Self ; }
};
}
