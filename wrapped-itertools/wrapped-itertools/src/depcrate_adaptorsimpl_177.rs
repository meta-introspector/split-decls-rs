// Generated macro for impl_177 (impl)
macro_rules! Depcrate_adaptorsimpl_177 {
() => {
// Module: crate::adaptors
// Provides: {"impl_177"}
// Dependencies: {}
impl < I , F , T , U , E > FusedIterator for FilterMapOk < I , F > where I : FusedIterator < Item = Result < T , E > > , F : FnMut (T) -> Option < U > , { }
};
}
