// Generated macro for impl_326 (impl)
macro_rules! Depcrateimpl_326 {
() => {
// Module: crate
// Provides: {"impl_326"}
// Dependencies: {}
impl CSlice < '_ > { # [doc = " Returns a raw pointer to the value, which is safe to pass over FFI."] pub fn as_ptr < T > (& self) -> * const T { if self . 0 . is_empty () { core :: ptr :: null () } else { self . 0 . as_ptr () as * const T } } pub fn len (& self) -> usize { self . 0 . len () } }
};
}
