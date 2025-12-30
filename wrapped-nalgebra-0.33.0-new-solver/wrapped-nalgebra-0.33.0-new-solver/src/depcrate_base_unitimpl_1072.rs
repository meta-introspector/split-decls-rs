// Generated macro for impl_1072 (impl)
macro_rules! Depcrate_base_unitimpl_1072 {
() => {
// Module: crate::base::unit
// Provides: {"impl_1072"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for Unit < T > { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { self . value . fmt (formatter) } }
};
}
