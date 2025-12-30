// Generated macro for NewObjectCacheFn (type)
macro_rules! Depcrate_cacheNewObjectCacheFn {
() => {
// Module: crate::cache
// Provides: {"NewObjectCacheFn"}
// Dependencies: {}
# [doc = " A constructor for boxed object caches."] pub type NewObjectCacheFn = dyn Fn () -> Box < ObjectCache > + Send + Sync + 'static ;
};
}
