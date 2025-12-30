// Generated macro for impl_362 (impl)
macro_rules! Depcrate_vecimpl_362 {
() => {
// Module: crate::vec
// Provides: {"impl_362"}
// Dependencies: {}
impl < T , LenT : LenType , const N : usize > Iterator for IntoIter < T , N , LenT > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { if self . next < self . vec . len { let item = unsafe { self . vec . buffer . buffer . get_unchecked_mut (self . next . into_usize ()) . as_ptr () . read () } ; self . next += LenT :: one () ; Some (item) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
};
}
