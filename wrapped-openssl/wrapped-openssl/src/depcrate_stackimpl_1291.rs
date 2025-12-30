// Generated macro for impl_1291 (impl)
macro_rules! Depcrate_stackimpl_1291 {
() => {
// Module: crate::stack
// Provides: {"impl_1291"}
// Dependencies: {}
impl < 'a , T : Stackable > Iterator for IterMut < 'a , T > { type Item = & 'a mut T :: Ref ; fn next (& mut self) -> Option < & 'a mut T :: Ref > { unsafe { self . idxs . next () . map (| i | T :: Ref :: from_ptr_mut (OPENSSL_sk_value (self . stack . as_stack () , i) as * mut _)) } } fn size_hint (& self) -> (usize , Option < usize >) { self . idxs . size_hint () } }
};
}
