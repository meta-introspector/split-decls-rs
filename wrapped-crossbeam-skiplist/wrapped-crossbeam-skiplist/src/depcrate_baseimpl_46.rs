// Generated macro for impl_46 (impl)
macro_rules! Depcrate_baseimpl_46 {
() => {
// Module: crate::base
// Provides: {"impl_46"}
// Dependencies: {}
impl < K , V > Clone for RefEntry < '_ , K , V > { fn clone (& self) -> Self { unsafe { Node :: try_increment (self . node) ; } Self { parent : self . parent , node : self . node , } } }
};
}
