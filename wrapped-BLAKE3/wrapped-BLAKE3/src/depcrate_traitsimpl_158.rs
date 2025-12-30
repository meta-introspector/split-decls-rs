// Generated macro for impl_158 (impl)
macro_rules! Depcrate_traitsimpl_158 {
() => {
// Module: crate::traits
// Provides: {"impl_158"}
// Dependencies: {}
impl digest :: KeyInit for Hasher { # [inline] fn new (key : & digest :: Key < Self >) -> Self { let key_bytes : [u8 ; 32] = (* key) . into () ; Hasher :: new_keyed (& key_bytes) } }
};
}
