// Generated macro for impl_323 (impl)
macro_rules! Depcrate_flatten_okimpl_323 {
() => {
// Module: crate::flatten_ok
// Provides: {"impl_323"}
// Dependencies: {}
impl < I , T , E > Clone for FlattenOk < I , T , E > where I : Iterator < Item = Result < T , E > > + Clone , T : IntoIterator , T :: IntoIter : Clone , { clone_fields ! (iter , inner_front , inner_back) ; }
};
}
