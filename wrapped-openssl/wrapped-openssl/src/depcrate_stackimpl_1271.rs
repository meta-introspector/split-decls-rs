// Generated macro for impl_1271 (impl)
macro_rules! Depcrate_stackimpl_1271 {
() => {
// Module: crate::stack
// Provides: {"impl_1271"}
// Dependencies: {}
impl < T : Stackable > Drop for IntoIter < T > { fn drop (& mut self) { unsafe { # [allow (clippy :: while_let_on_iterator)] while let Some (_) = self . next () { } OPENSSL_sk_free (self . stack as * mut _) ; } } }
};
}
