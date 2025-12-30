// Generated macro for impl_191 (impl)
macro_rules! Depcrate_hir_literalimpl_191 {
() => {
// Module: crate::hir::literal
// Provides: {"impl_191"}
// Dependencies: {}
impl FromIterator < Literal > for Seq { fn from_iter < T : IntoIterator < Item = Literal > > (it : T) -> Seq { let mut seq = Seq :: empty () ; for literal in it { seq . push (literal) ; } seq } }
};
}
