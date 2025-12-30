// Generated macro for impl_1272 (impl)
macro_rules! Depcrate_stackimpl_1272 {
() => {
// Module: crate::stack
// Provides: {"impl_1272"}
// Dependencies: {}
impl < T : Stackable > Iterator for IntoIter < T > { type Item = T ; fn next (& mut self) -> Option < T > { unsafe { self . idxs . next () . map (| i | T :: from_ptr (OPENSSL_sk_value (self . stack as * mut _ , i) as * mut _)) } } fn size_hint (& self) -> (usize , Option < usize >) { self . idxs . size_hint () } }
};
}
