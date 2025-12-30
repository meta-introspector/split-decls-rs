// Generated macro for impl_792 (impl)
macro_rules! Depcrate_kemimpl_792 {
() => {
// Module: crate::kem
// Provides: {"impl_792"}
// Dependencies: {}
impl < Id > Debug for DecapsulationKey < Id > where Id : AlgorithmIdentifier , { fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { f . debug_struct ("DecapsulationKey") . field ("algorithm" , & self . algorithm) . finish_non_exhaustive () } }
};
}
