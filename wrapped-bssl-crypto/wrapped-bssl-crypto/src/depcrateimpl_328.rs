// Generated macro for impl_328 (impl)
macro_rules! Depcrateimpl_328 {
() => {
// Module: crate
// Provides: {"impl_328"}
// Dependencies: {}
impl CSliceMut < '_ > { # [doc = " Returns a raw pointer to the value, which is safe to pass over FFI."] pub fn as_mut_ptr < T > (& mut self) -> * mut T { if self . 0 . is_empty () { core :: ptr :: null_mut () } else { self . 0 . as_mut_ptr () as * mut T } } pub fn len (& self) -> usize { self . 0 . len () } }
};
}
