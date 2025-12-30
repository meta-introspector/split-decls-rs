// Generated macro for impl_879 (impl)
macro_rules! Depcrate_iter_map_withimpl_879 {
() => {
// Module: crate::iter::map_with
// Provides: {"impl_879"}
// Dependencies: {}
impl < 'f , I , U , F , R > DoubleEndedIterator for MapWithIter < 'f , I , U , F > where I : DoubleEndedIterator , F : Fn (& mut U , I :: Item) -> R + Sync , R : Send , { fn next_back (& mut self) -> Option < R > { let item = self . base . next_back () ? ; Some ((self . map_op) (& mut self . item , item)) } }
};
}
