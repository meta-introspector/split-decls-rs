// Generated macro for impl_718 (impl)
macro_rules! Depcrate_unique_implimpl_718 {
() => {
// Module: crate::unique_impl
// Provides: {"impl_718"}
// Dependencies: {}
impl < I , V , F > DoubleEndedIterator for UniqueBy < I , V , F > where I : DoubleEndedIterator , V : Eq + Hash , F : FnMut (& I :: Item) -> V , { fn next_back (& mut self) -> Option < Self :: Item > { let Self { iter , used , f } = self ; iter . rfind (| v | used . insert (f (v) , ()) . is_none ()) } }
};
}
