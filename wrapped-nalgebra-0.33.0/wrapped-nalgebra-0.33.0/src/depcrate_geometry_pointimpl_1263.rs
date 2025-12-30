// Generated macro for impl_1263 (impl)
macro_rules! Depcrate_geometry_pointimpl_1263 {
() => {
// Module: crate::geometry::point
// Provides: {"impl_1263"}
// Dependencies: {}
impl < T : Scalar + fmt :: Debug , D : DimName > fmt :: Debug for OPoint < T , D > where DefaultAllocator : Allocator < D > , { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { self . coords . as_slice () . fmt (formatter) } }
};
}
