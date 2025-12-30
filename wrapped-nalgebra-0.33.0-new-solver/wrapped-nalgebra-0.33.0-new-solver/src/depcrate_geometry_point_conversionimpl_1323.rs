// Generated macro for impl_1323 (impl)
macro_rules! Depcrate_geometry_point_conversionimpl_1323 {
() => {
// Module: crate::geometry::point_conversion
// Provides: {"impl_1323"}
// Dependencies: {}
impl < T : Scalar + Copy + PrimitiveSimdValue , const D : usize > From < [Point < T :: Element , D > ; 4] > for Point < T , D > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 4] > , T :: Element : Scalar + Copy , < DefaultAllocator as Allocator < Const < D > > > :: Buffer < T :: Element > : Copy , { # [inline] fn from (arr : [Point < T :: Element , D > ; 4]) -> Self { Self :: from (OVector :: from ([arr [0] . coords , arr [1] . coords , arr [2] . coords , arr [3] . coords ,])) } }
};
}
