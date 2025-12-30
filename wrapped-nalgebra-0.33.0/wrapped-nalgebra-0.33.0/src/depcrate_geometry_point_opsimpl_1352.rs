// Generated macro for impl_1352 (impl)
macro_rules! Depcrate_geometry_point_opsimpl_1352 {
() => {
// Module: crate::geometry::point_ops
// Provides: {"impl_1352"}
// Dependencies: {}
impl < 'a , T : Scalar + ClosedNeg , D : DimName > Neg for & 'a OPoint < T , D > where DefaultAllocator : Allocator < D > , { type Output = OPoint < T , D > ; # [inline] fn neg (self) -> Self :: Output { Self :: Output :: from (- & self . coords) } }
};
}
