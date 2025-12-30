// Generated macro for _test_lifetimes (function)
macro_rules! Depcrate_test_lifetimes {
() => {
// Module: crate
// Provides: {"_test_lifetimes"}
// Dependencies: {}
# [doc = " Doctests for what should *not* compile"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " let mut cache = lru::LruCache::<u32, u32>::unbounded();"] # [doc = " let _: &'static u32 = cache.get_or_insert(0, || 92);"] # [doc = " ```"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " let mut cache = lru::LruCache::<u32, u32>::unbounded();"] # [doc = " let _: Option<(&'static u32, _)> = cache.peek_lru();"] # [doc = " let _: Option<(_, &'static u32)> = cache.peek_lru();"] # [doc = " ```"] fn _test_lifetimes () { }
};
}
