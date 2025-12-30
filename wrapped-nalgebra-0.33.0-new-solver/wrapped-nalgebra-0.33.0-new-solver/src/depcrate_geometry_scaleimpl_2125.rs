// Generated macro for impl_2125 (impl)
macro_rules! Depcrate_geometry_scaleimpl_2125 {
() => {
// Module: crate::geometry::scale
// Provides: {"impl_2125"}
// Dependencies: {}
impl < T : fmt :: Debug , const D : usize > fmt :: Debug for Scale < T , D > { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { self . vector . as_slice () . fmt (formatter) } }
};
}
