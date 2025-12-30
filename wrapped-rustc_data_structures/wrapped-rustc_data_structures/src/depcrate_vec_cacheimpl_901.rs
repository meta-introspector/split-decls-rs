// Generated macro for impl_901 (impl)
macro_rules! Depcrate_vec_cacheimpl_901 {
() => {
// Module: crate::vec_cache
// Provides: {"impl_901"}
// Dependencies: {}
unsafe impl < K : Idx , # [may_dangle] V , I > Drop for VecCache < K , V , I > { fn drop (& mut self) { assert ! (! std :: mem :: needs_drop ::< K > ()) ; assert ! (! std :: mem :: needs_drop ::< V > ()) ; for (idx , bucket) in self . buckets . iter () . enumerate () { let bucket = bucket . load (Ordering :: Acquire) ; if ! bucket . is_null () { let layout = std :: alloc :: Layout :: array :: < Slot < V > > (ENTRIES_BY_BUCKET [idx]) . unwrap () ; unsafe { std :: alloc :: dealloc (bucket . cast () , layout) ; } } } for (idx , bucket) in self . present . iter () . enumerate () { let bucket = bucket . load (Ordering :: Acquire) ; if ! bucket . is_null () { let layout = std :: alloc :: Layout :: array :: < Slot < () > > (ENTRIES_BY_BUCKET [idx]) . unwrap () ; unsafe { std :: alloc :: dealloc (bucket . cast () , layout) ; } } } } }
};
}
