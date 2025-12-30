// Generated macro for impl_341 (impl)
macro_rules! Depcrateimpl_341 {
() => {
// Module: crate
// Provides: {"impl_341"}
// Dependencies: {}
impl AsRef < [u8] > for Buffer { fn as_ref (& self) -> & [u8] { if self . len == 0 { return & [] ; } unsafe { core :: slice :: from_raw_parts (self . ptr , self . len) } } }
};
}
