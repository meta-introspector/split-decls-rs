// Generated macro for impl_1262 (impl)
macro_rules! Depcrate_stackimpl_1262 {
() => {
// Module: crate::stack
// Provides: {"impl_1262"}
// Dependencies: {}
impl < T : Stackable > Drop for Stack < T > { fn drop (& mut self) { unsafe { while self . pop () . is_some () { } OPENSSL_sk_free (self . 0 as * mut _) ; } } }
};
}
