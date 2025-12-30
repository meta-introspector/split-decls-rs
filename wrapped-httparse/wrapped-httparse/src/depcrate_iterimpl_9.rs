// Generated macro for impl_9 (impl)
macro_rules! Depcrate_iterimpl_9 {
() => {
// Module: crate::iter
// Provides: {"impl_9"}
// Dependencies: {}
impl AsRef < [u8] > for Bytes < '_ > { # [inline] fn as_ref (& self) -> & [u8] { unsafe { slice_from_ptr_range (self . cursor , self . end) } } }
};
}
