// Generated macro for impl_1292 (impl)
macro_rules! Depcrate_stackimpl_1292 {
() => {
// Module: crate::stack
// Provides: {"impl_1292"}
// Dependencies: {}
impl < 'a , T : Stackable > DoubleEndedIterator for IterMut < 'a , T > { fn next_back (& mut self) -> Option < & 'a mut T :: Ref > { unsafe { self . idxs . next_back () . map (| i | T :: Ref :: from_ptr_mut (OPENSSL_sk_value (self . stack . as_stack () , i) as * mut _)) } } }
};
}
