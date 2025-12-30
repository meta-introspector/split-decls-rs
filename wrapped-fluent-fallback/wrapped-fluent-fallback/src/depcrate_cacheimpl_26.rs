// Generated macro for impl_26 (impl)
macro_rules! Depcrate_cacheimpl_26 {
() => {
// Module: crate::cache
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'a , I , R > Iterator for CacheIter < 'a , I , R > where I : Iterator , { type Item = & 'a I :: Item ; fn next (& mut self) -> Option < Self :: Item > { let cache_len = self . cache . len () ; match self . curr . cmp (& cache_len) { Ordering :: Less => { self . curr += 1 ; self . cache . get (self . curr - 1) } Ordering :: Equal => { let item = self . cache . iter . borrow_mut () . next () ; self . curr += 1 ; if let Some (item) = item { Some (self . cache . push_get (item)) } else { None } } Ordering :: Greater => { None } } } }
};
}
