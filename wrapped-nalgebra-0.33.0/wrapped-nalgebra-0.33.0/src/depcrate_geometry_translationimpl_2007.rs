// Generated macro for impl_2007 (impl)
macro_rules! Depcrate_geometry_translationimpl_2007 {
() => {
// Module: crate::geometry::translation
// Provides: {"impl_2007"}
// Dependencies: {}
impl < T : fmt :: Debug , const D : usize > fmt :: Debug for Translation < T , D > { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { self . vector . as_slice () . fmt (formatter) } }
};
}
