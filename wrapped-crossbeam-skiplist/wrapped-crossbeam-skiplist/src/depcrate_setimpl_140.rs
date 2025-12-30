// Generated macro for impl_140 (impl)
macro_rules! Depcrate_setimpl_140 {
() => {
// Module: crate::set
// Provides: {"impl_140"}
// Dependencies: {}
impl < 'a , Q , R , T > DoubleEndedIterator for Range < 'a , Q , R , T > where T : Ord + Comparable < Q > , R : RangeBounds < Q > , Q : ? Sized , { fn next_back (& mut self) -> Option < Entry < 'a , T > > { self . inner . next_back () . map (Entry :: new) } }
};
}
