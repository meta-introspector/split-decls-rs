// Generated macro for impl_109 (impl)
macro_rules! Depcrate_mapimpl_109 {
() => {
// Module: crate::map
// Provides: {"impl_109"}
// Dependencies: {}
impl < Q , R , K , V > Drop for Range < '_ , Q , R , K , V > where K : Ord + Comparable < Q > , R : RangeBounds < Q > , Q : ? Sized , { fn drop (& mut self) { let guard = & epoch :: pin () ; self . inner . drop_impl (guard) ; } }
};
}
