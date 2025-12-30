// Generated macro for impl_139 (impl)
macro_rules! Depcrate_setimpl_139 {
() => {
// Module: crate::set
// Provides: {"impl_139"}
// Dependencies: {}
impl < 'a , Q , R , T > Iterator for Range < 'a , Q , R , T > where T : Ord + Comparable < Q > , R : RangeBounds < Q > , Q : ? Sized , { type Item = Entry < 'a , T > ; fn next (& mut self) -> Option < Entry < 'a , T > > { self . inner . next () . map (Entry :: new) } }
};
}
