// Generated macro for impl_140 (impl)
macro_rules! Depcrate_packed_extimpl_140 {
() => {
// Module: crate::packed::ext
// Provides: {"impl_140"}
// Dependencies: {}
impl < T > Pointer for * const T { unsafe fn distance (self , origin : * const T) -> usize { usize :: try_from (self . offset_from (origin)) . unwrap_unchecked () } fn as_usize (self) -> usize { self as usize } }
};
}
