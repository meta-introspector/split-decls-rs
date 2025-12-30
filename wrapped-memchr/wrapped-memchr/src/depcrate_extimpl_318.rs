// Generated macro for impl_318 (impl)
macro_rules! Depcrate_extimpl_318 {
() => {
// Module: crate::ext
// Provides: {"impl_318"}
// Dependencies: {}
impl < T > Pointer for * const T { unsafe fn distance (self , origin : * const T) -> usize { usize :: try_from (self . offset_from (origin)) . unwrap_unchecked () } fn as_usize (self) -> usize { self as usize } }
};
}
