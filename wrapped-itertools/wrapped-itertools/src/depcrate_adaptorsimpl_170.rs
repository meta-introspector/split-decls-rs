// Generated macro for impl_170 (impl)
macro_rules! Depcrate_adaptorsimpl_170 {
() => {
// Module: crate::adaptors
// Provides: {"impl_170"}
// Dependencies: {}
impl < I , F , T , E > FusedIterator for FilterOk < I , F > where I : FusedIterator < Item = Result < T , E > > , F : FnMut (& T) -> bool , { }
};
}
