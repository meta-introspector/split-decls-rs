// Generated macro for impl_45 (impl)
macro_rules! Depcrate_generateimpl_45 {
() => {
// Module: crate::generate
// Provides: {"impl_45"}
// Dependencies: {}
impl < I : Iterator > Iterator for KnownSize < I > { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { let next = self . iter . next () ; if next . is_some () { self . current += 1 ; return next ; } assert_eq ! (self . current , self . total , "total items did not match expected") ; None } fn size_hint (& self) -> (usize , Option < usize >) { let remaining = usize :: try_from (self . total - self . current) . unwrap () ; (remaining , Some (remaining)) } }
};
}
