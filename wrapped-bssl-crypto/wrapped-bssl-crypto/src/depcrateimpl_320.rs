// Generated macro for impl_320 (impl)
macro_rules! Depcrateimpl_320 {
() => {
// Module: crate
// Provides: {"impl_320"}
// Dependencies: {}
impl < T , const N : usize > FfiSlice < T > for [T ; N] { fn as_ffi_ptr (& self) -> * const T { if N == 0 { core :: ptr :: null () } else { self . as_ptr () } } }
};
}
