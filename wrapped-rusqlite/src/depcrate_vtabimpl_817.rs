// Generated macro for impl_817 (impl)
macro_rules! Depcrate_vtabimpl_817 {
() => {
// Module: crate::vtab
// Provides: {"impl_817"}
// Dependencies: {}
impl < 'a > Iterator for IndexConstraintIter < 'a > { type Item = IndexConstraint < 'a > ; # [inline] fn next (& mut self) -> Option < IndexConstraint < 'a > > { self . iter . next () . map (IndexConstraint) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
