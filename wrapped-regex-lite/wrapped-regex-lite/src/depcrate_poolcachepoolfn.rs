// Generated macro for CachePoolFn (type)
macro_rules! Depcrate_poolCachePoolFn {
() => {
// Module: crate::pool
// Provides: {"CachePoolFn"}
// Dependencies: {}
# [doc = " The type of the closure we use to create new caches. We need to spell out"] # [doc = " all of the marker traits or else we risk leaking !MARKER impls."] pub (crate) type CachePoolFn = Box < dyn Fn () -> pikevm :: Cache + Send + Sync + UnwindSafe + RefUnwindSafe > ;
};
}
