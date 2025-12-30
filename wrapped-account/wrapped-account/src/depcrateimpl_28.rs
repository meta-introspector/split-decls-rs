// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl ReadableAccount for Ref < '_ , AccountSharedData > { fn lamports (& self) -> u64 { self . lamports } fn data (& self) -> & [u8] { & self . data } fn owner (& self) -> & Pubkey { & self . owner } fn executable (& self) -> bool { self . executable } fn rent_epoch (& self) -> Epoch { self . rent_epoch } fn to_account_shared_data (& self) -> AccountSharedData { AccountSharedData { lamports : self . lamports () , data : Arc :: clone (& self . data) , owner : * self . owner () , executable : self . executable () , rent_epoch : self . rent_epoch () , } } }
};
}
