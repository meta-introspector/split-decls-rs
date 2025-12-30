// Generated macro for impl_1653 (impl)
macro_rules! Depcrate_streamimpl_1653 {
() => {
// Module: crate::stream
// Provides: {"impl_1653"}
// Dependencies: {}
impl StreamIter { # [inline] fn from (streams : & StreamIdHashSet) -> Self { StreamIter { streams : streams . iter () . copied () . collect () , index : 0 , } } }
};
}
