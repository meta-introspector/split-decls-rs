// Generated macro for impl_325 (impl)
macro_rules! Depcrate_flatten_okimpl_325 {
() => {
// Module: crate::flatten_ok
// Provides: {"impl_325"}
// Dependencies: {}
# [doc = " Only the iterator being flattened needs to implement [`FusedIterator`]."] impl < I , T , E > FusedIterator for FlattenOk < I , T , E > where I : FusedIterator < Item = Result < T , E > > , T : IntoIterator , { }
};
}
