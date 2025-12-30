// Generated macro for impl_1516 (impl)
macro_rules! Depcrate_geometry_quaternionimpl_1516 {
() => {
// Module: crate::geometry::quaternion
// Provides: {"impl_1516"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for Quaternion < T > { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { self . coords . as_slice () . fmt (formatter) } }
};
}
