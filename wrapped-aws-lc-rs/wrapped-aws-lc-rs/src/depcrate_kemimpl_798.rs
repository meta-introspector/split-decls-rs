// Generated macro for impl_798 (impl)
macro_rules! Depcrate_kemimpl_798 {
() => {
// Module: crate::kem
// Provides: {"impl_798"}
// Dependencies: {}
impl < Id > Debug for EncapsulationKey < Id > where Id : AlgorithmIdentifier , { fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("EncapsulationKey") . field ("algorithm" , & self . algorithm) . finish_non_exhaustive () } }
};
}
