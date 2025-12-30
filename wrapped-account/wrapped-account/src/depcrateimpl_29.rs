// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl ReadableAccount for Ref < '_ , Account > { fn lamports (& self) -> u64 { self . lamports } fn data (& self) -> & [u8] { & self . data } fn owner (& self) -> & Pubkey { & self . owner } fn executable (& self) -> bool { self . executable } fn rent_epoch (& self) -> Epoch { self . rent_epoch } }
};
}
