// Generated macro for impl_1287 (impl)
macro_rules! Depcrate_stackimpl_1287 {
() => {
// Module: crate::stack
// Provides: {"impl_1287"}
// Dependencies: {}
impl < 'a , T : Stackable > Iterator for Iter < 'a , T > { type Item = & 'a T :: Ref ; fn next (& mut self) -> Option < & 'a T :: Ref > { unsafe { self . idxs . next () . map (| i | T :: Ref :: from_ptr (OPENSSL_sk_value (self . stack . as_stack () , i) as * mut _)) } } fn size_hint (& self) -> (usize , Option < usize >) { self . idxs . size_hint () } }
};
}
