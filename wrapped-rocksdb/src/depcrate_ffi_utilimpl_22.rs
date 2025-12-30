// Generated macro for impl_22 (impl)
macro_rules! Depcrate_ffi_utilimpl_22 {
() => {
// Module: crate::ffi_util
// Provides: {"impl_22"}
// Dependencies: {}
impl AsRef < [u8] > for CSlice { fn as_ref (& self) -> & [u8] { unsafe { std :: slice :: from_raw_parts (self . data as * const u8 , self . len) } } }
};
}
