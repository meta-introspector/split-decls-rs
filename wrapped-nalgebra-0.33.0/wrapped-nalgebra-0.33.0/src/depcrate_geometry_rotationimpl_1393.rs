// Generated macro for impl_1393 (impl)
macro_rules! Depcrate_geometry_rotationimpl_1393 {
() => {
// Module: crate::geometry::rotation
// Provides: {"impl_1393"}
// Dependencies: {}
impl < T : fmt :: Debug , const D : usize > fmt :: Debug for Rotation < T , D > { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { self . matrix . fmt (formatter) } }
};
}
