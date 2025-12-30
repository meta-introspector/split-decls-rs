// Generated macro for impl_719 (impl)
macro_rules! Depcrate_unique_implimpl_719 {
() => {
// Module: crate::unique_impl
// Provides: {"impl_719"}
// Dependencies: {}
impl < I , V , F > FusedIterator for UniqueBy < I , V , F > where I : FusedIterator , V : Eq + Hash , F : FnMut (& I :: Item) -> V , { }
};
}
