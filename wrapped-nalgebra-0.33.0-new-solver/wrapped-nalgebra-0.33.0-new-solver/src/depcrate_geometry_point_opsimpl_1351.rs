// Generated macro for impl_1351 (impl)
macro_rules! Depcrate_geometry_point_opsimpl_1351 {
() => {
// Module: crate::geometry::point_ops
// Provides: {"impl_1351"}
// Dependencies: {}
impl < T : Scalar + ClosedNeg , D : DimName > Neg for OPoint < T , D > where DefaultAllocator : Allocator < D > , { type Output = Self ; # [inline] fn neg (self) -> Self :: Output { Self :: Output :: from (- self . coords) } }
};
}
