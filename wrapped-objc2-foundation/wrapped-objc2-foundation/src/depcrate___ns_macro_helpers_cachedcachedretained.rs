// Generated macro for CachedRetained (struct)
macro_rules! Depcrate___ns_macro_helpers_cachedCachedRetained {
() => {
// Module: crate::__ns_macro_helpers::cached
// Provides: {"CachedRetained"}
// Dependencies: {}
# [doc = " Allows storing an `Retained` in a static and lazily loading it."] # [derive (Debug)] pub struct CachedRetained < T > { ptr : AtomicPtr < T > , }
};
}
