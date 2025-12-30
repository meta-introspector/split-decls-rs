// Generated macro for impl_24 (impl)
macro_rules! Depcrate_cacheimpl_24 {
() => {
// Module: crate::cache
// Provides: {"impl_24"}
// Dependencies: {}
impl < I , R > Cache < I , R > where I : BundleIterator + Iterator , { pub fn prefetch (& self) { self . iter . borrow_mut () . prefetch_sync () ; } }
};
}
