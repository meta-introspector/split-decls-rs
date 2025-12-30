// Generated macro for impl_319 (impl)
macro_rules! Depcrateimpl_319 {
() => {
// Module: crate
// Provides: {"impl_319"}
// Dependencies: {}
impl < T > FfiSlice < T > for [T] { fn as_ffi_ptr (& self) -> * const T { if self . is_empty () { core :: ptr :: null () } else { self . as_ptr () } } }
};
}
