// Generated macro for impl_1288 (impl)
macro_rules! Depcrate_stackimpl_1288 {
() => {
// Module: crate::stack
// Provides: {"impl_1288"}
// Dependencies: {}
impl < 'a , T : Stackable > DoubleEndedIterator for Iter < 'a , T > { fn next_back (& mut self) -> Option < & 'a T :: Ref > { unsafe { self . idxs . next_back () . map (| i | T :: Ref :: from_ptr (OPENSSL_sk_value (self . stack . as_stack () , i) as * mut _)) } } }
};
}
