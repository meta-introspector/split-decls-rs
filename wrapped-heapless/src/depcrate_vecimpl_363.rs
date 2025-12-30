// Generated macro for impl_363 (impl)
macro_rules! Depcrate_vecimpl_363 {
() => {
// Module: crate::vec
// Provides: {"impl_363"}
// Dependencies: {}
impl < T , LenT : LenType , const N : usize > DoubleEndedIterator for IntoIter < T , N , LenT > { fn next_back (& mut self) -> Option < Self :: Item > { if self . next < self . vec . len { let item = unsafe { self . vec . pop_unchecked () } ; Some (item) } else { None } } }
};
}
