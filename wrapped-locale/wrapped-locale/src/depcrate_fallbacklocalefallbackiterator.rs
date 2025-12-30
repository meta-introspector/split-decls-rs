// Generated macro for LocaleFallbackIterator (struct)
macro_rules! Depcrate_fallbackLocaleFallbackIterator {
() => {
// Module: crate::fallback
// Provides: {"LocaleFallbackIterator"}
// Dependencies: {}
# [doc = " Iteration type for locale fallback operations."] # [doc = ""] # [doc = " Because the `Iterator` trait does not allow items to borrow from the iterator, this class does"] # [doc = " not implement that trait. Instead, use `.step()` and `.get()`."] # [derive (Debug)] pub struct LocaleFallbackIterator < 'a > { current : DataLocale , inner : LocaleFallbackIteratorInner < 'a > , }
};
}
