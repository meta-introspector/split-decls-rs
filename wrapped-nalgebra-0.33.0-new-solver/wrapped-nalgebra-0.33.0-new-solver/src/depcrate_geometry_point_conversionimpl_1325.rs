// Generated macro for impl_1325 (impl)
macro_rules! Depcrate_geometry_point_conversionimpl_1325 {
() => {
// Module: crate::geometry::point_conversion
// Provides: {"impl_1325"}
// Dependencies: {}
impl < T : Scalar + Copy + PrimitiveSimdValue , const D : usize > From < [Point < T :: Element , D > ; 16] > for Point < T , D > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 16] > , T :: Element : Scalar + Copy , < DefaultAllocator as Allocator < Const < D > > > :: Buffer < T :: Element > : Copy , { # [inline] fn from (arr : [Point < T :: Element , D > ; 16]) -> Self { Self :: from (OVector :: from ([arr [0] . coords , arr [1] . coords , arr [2] . coords , arr [3] . coords , arr [4] . coords , arr [5] . coords , arr [6] . coords , arr [7] . coords , arr [8] . coords , arr [9] . coords , arr [10] . coords , arr [11] . coords , arr [12] . coords , arr [13] . coords , arr [14] . coords , arr [15] . coords ,])) } }
};
}
