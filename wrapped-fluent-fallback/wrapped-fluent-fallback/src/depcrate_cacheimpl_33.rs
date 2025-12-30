// Generated macro for impl_33 (impl)
macro_rules! Depcrate_cacheimpl_33 {
() => {
// Module: crate::cache
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'a , S , R > Stream for AsyncCacheStream < 'a , S , R > where S : Stream , { type Item = & 'a S :: Item ; fn poll_next (mut self : std :: pin :: Pin < & mut Self > , cx : & mut std :: task :: Context < '_ > ,) -> Poll < Option < Self :: Item > > { let cache_len = self . cache . len () ; match self . curr . cmp (& cache_len) { Ordering :: Less => { self . curr += 1 ; self . cache . get (self . curr - 1) } Ordering :: Equal => { let item = ready ! (self . cache . poll_next_item (cx)) ; self . curr += 1 ; if let Some (item) = item { Some (self . cache . push_get (item)) . into () } else { None . into () } } Ordering :: Greater => { None . into () } } } }
};
}
