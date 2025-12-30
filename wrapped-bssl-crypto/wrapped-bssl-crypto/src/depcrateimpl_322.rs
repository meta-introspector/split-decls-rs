// Generated macro for impl_322 (impl)
macro_rules! Depcrateimpl_322 {
() => {
// Module: crate
// Provides: {"impl_322"}
// Dependencies: {}
impl FfiMutSlice for [u8] { fn as_mut_ffi_ptr (& mut self) -> * mut u8 { if self . is_empty () { core :: ptr :: null_mut () } else { self . as_mut_ptr () } } }
};
}
