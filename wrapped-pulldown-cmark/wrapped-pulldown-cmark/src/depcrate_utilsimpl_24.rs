// Generated macro for impl_24 (impl)
macro_rules! Depcrate_utilsimpl_24 {
() => {
// Module: crate::utils
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a , I > TextMergeStream < 'a , I > where I : Iterator < Item = Event < 'a > > , { pub fn new (iter : I) -> Self { Self { inner : TextMergeWithOffset :: new (DummyOffsets (iter)) , } } }
};
}
