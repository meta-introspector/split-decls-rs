// Generated macro for ReadableAccount (trait)
macro_rules! DepcrateReadableAccount {
() => {
// Module: crate
// Provides: {"ReadableAccount"}
// Dependencies: {}
pub trait ReadableAccount : Sized { fn lamports (& self) -> u64 ; fn data (& self) -> & [u8] ; fn owner (& self) -> & Pubkey ; fn executable (& self) -> bool ; fn rent_epoch (& self) -> Epoch ; # [deprecated (since = "3.2.0")] fn to_account_shared_data (& self) -> AccountSharedData { AccountSharedData :: create (self . lamports () , self . data () . to_vec () , * self . owner () , self . executable () , self . rent_epoch () ,) } }
};
}
