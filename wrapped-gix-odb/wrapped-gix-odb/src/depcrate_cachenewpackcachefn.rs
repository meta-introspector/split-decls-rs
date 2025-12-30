// Generated macro for NewPackCacheFn (type)
macro_rules! Depcrate_cacheNewPackCacheFn {
() => {
// Module: crate::cache
// Provides: {"NewPackCacheFn"}
// Dependencies: {}
# [doc = " A constructor for boxed pack caches."] pub type NewPackCacheFn = dyn Fn () -> Box < PackCache > + Send + Sync + 'static ;
};
}
