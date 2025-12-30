// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl ReadableAccount for AccountSharedData { fn lamports (& self) -> u64 { self . lamports } fn data (& self) -> & [u8] { & self . data } fn owner (& self) -> & Pubkey { & self . owner } fn executable (& self) -> bool { self . executable } fn rent_epoch (& self) -> Epoch { self . rent_epoch } fn to_account_shared_data (& self) -> AccountSharedData { self . clone () } }
};
}
