// Generated macro for impl_717 (impl)
macro_rules! Depcrate_unique_implimpl_717 {
() => {
// Module: crate::unique_impl
// Provides: {"impl_717"}
// Dependencies: {}
impl < I , V , F > Iterator for UniqueBy < I , V , F > where I : Iterator , V : Eq + Hash , F : FnMut (& I :: Item) -> V , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { let Self { iter , used , f } = self ; iter . find (| v | used . insert (f (v) , ()) . is_none ()) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let (low , hi) = self . iter . size_hint () ; ((low > 0 && self . used . is_empty ()) as usize , hi) } fn count (self) -> usize { let mut key_f = self . f ; count_new_keys (self . used , self . iter . map (move | elt | key_f (& elt))) } }
};
}
