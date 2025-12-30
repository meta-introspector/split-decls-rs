// Generated macro for impl_1324 (impl)
macro_rules! Depcrate_geometry_point_conversionimpl_1324 {
() => {
// Module: crate::geometry::point_conversion
// Provides: {"impl_1324"}
// Dependencies: {}
impl < T : Scalar + Copy + PrimitiveSimdValue , const D : usize > From < [Point < T :: Element , D > ; 8] > for Point < T , D > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 8] > , T :: Element : Scalar + Copy , < DefaultAllocator as Allocator < Const < D > > > :: Buffer < T :: Element > : Copy , { # [inline] fn from (arr : [Point < T :: Element , D > ; 8]) -> Self { Self :: from (OVector :: from ([arr [0] . coords , arr [1] . coords , arr [2] . coords , arr [3] . coords , arr [4] . coords , arr [5] . coords , arr [6] . coords , arr [7] . coords ,])) } }
};
}
