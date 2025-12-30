// Generated macro for impl_36 (impl)
macro_rules! Depcrate_literalimpl_36 {
() => {
// Module: crate::literal
// Provides: {"impl_36"}
// Dependencies: {}
impl FromIterator < Literal > for TSeq { fn from_iter < T : IntoIterator < Item = Literal > > (it : T) -> TSeq { TSeq { seq : Seq :: from_iter (it) , prefix : true } } }
};
}
