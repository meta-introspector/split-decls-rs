// Generated macro for impl_13 (impl)
macro_rules! Depcrate_cpiimpl_13 {
() => {
// Module: crate::cpi
// Provides: {"impl_13"}
// Dependencies: {}
impl Deref for Seed < '_ > { type Target = [u8] ; fn deref (& self) -> & Self :: Target { unsafe { from_raw_parts (self . seed , self . len as usize) } } }
};
}
