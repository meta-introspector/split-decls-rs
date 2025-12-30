// Generated macro for impl_880 (impl)
macro_rules! Depcrate_iter_map_withimpl_880 {
() => {
// Module: crate::iter::map_with
// Provides: {"impl_880"}
// Dependencies: {}
impl < 'f , I , U , F , R > ExactSizeIterator for MapWithIter < 'f , I , U , F > where I : ExactSizeIterator , F : Fn (& mut U , I :: Item) -> R + Sync , R : Send , { }
};
}
