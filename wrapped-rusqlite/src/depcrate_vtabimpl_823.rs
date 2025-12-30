// Generated macro for impl_823 (impl)
macro_rules! Depcrate_vtabimpl_823 {
() => {
// Module: crate::vtab
// Provides: {"impl_823"}
// Dependencies: {}
impl < 'a > Iterator for OrderByIter < 'a > { type Item = OrderBy < 'a > ; # [inline] fn next (& mut self) -> Option < OrderBy < 'a > > { self . iter . next () . map (OrderBy) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
