// Generated macro for impl_1273 (impl)
macro_rules! Depcrate_stackimpl_1273 {
() => {
// Module: crate::stack
// Provides: {"impl_1273"}
// Dependencies: {}
impl < T : Stackable > DoubleEndedIterator for IntoIter < T > { fn next_back (& mut self) -> Option < T > { unsafe { self . idxs . next_back () . map (| i | T :: from_ptr (OPENSSL_sk_value (self . stack as * mut _ , i) as * mut _)) } } }
};
}
