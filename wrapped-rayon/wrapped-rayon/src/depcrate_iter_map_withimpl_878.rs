// Generated macro for impl_878 (impl)
macro_rules! Depcrate_iter_map_withimpl_878 {
() => {
// Module: crate::iter::map_with
// Provides: {"impl_878"}
// Dependencies: {}
impl < 'f , I , U , F , R > Iterator for MapWithIter < 'f , I , U , F > where I : Iterator , F : Fn (& mut U , I :: Item) -> R + Sync , R : Send , { type Item = R ; fn next (& mut self) -> Option < R > { let item = self . base . next () ? ; Some ((self . map_op) (& mut self . item , item)) } fn size_hint (& self) -> (usize , Option < usize >) { self . base . size_hint () } }
};
}
