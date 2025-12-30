// Generated macro for impl_324 (impl)
macro_rules! Depcrate_flatten_okimpl_324 {
() => {
// Module: crate::flatten_ok
// Provides: {"impl_324"}
// Dependencies: {}
impl < I , T , E > fmt :: Debug for FlattenOk < I , T , E > where I : Iterator < Item = Result < T , E > > + fmt :: Debug , T : IntoIterator , T :: IntoIter : fmt :: Debug , { debug_fmt_fields ! (FlattenOk , iter , inner_front , inner_back) ; }
};
}
