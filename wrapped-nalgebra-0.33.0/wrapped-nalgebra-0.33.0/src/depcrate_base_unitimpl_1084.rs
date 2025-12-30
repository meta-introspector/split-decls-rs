// Generated macro for impl_1084 (impl)
macro_rules! Depcrate_base_unitimpl_1084 {
() => {
// Module: crate::base::unit
// Provides: {"impl_1084"}
// Dependencies: {}
impl < T : Scalar + simba :: simd :: PrimitiveSimdValue , R : Dim , C : Dim > From < [Unit < OMatrix < T :: Element , R , C > > ; 2] > for Unit < OMatrix < T , R , C > > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 2] > , T :: Element : Scalar , DefaultAllocator : Allocator < R , C > , { # [inline] fn from (arr : [Unit < OMatrix < T :: Element , R , C > > ; 2]) -> Self { Self :: new_unchecked (OMatrix :: from ([arr [0] . clone () . into_inner () , arr [1] . clone () . into_inner () ,])) } }
};
}
