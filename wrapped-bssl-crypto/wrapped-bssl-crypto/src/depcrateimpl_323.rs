// Generated macro for impl_323 (impl)
macro_rules! Depcrateimpl_323 {
() => {
// Module: crate
// Provides: {"impl_323"}
// Dependencies: {}
impl < const N : usize > FfiMutSlice for [u8 ; N] { fn as_mut_ffi_ptr (& mut self) -> * mut u8 { if N == 0 { core :: ptr :: null_mut () } else { self . as_mut_ptr () } } }
};
}
