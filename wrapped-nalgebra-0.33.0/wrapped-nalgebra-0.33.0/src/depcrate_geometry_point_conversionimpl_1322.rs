// Generated macro for impl_1322 (impl)
macro_rules! Depcrate_geometry_point_conversionimpl_1322 {
() => {
// Module: crate::geometry::point_conversion
// Provides: {"impl_1322"}
// Dependencies: {}
impl < T : Scalar + Copy + PrimitiveSimdValue , const D : usize > From < [Point < T :: Element , D > ; 2] > for Point < T , D > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 2] > , T :: Element : Scalar + Copy , < DefaultAllocator as Allocator < Const < D > > > :: Buffer < T :: Element > : Copy , { # [inline] fn from (arr : [Point < T :: Element , D > ; 2]) -> Self { Self :: from (OVector :: from ([arr [0] . coords , arr [1] . coords])) } }
};
}
