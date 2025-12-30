// Generated macro for impl_1087 (impl)
macro_rules! Depcrate_base_unitimpl_1087 {
() => {
// Module: crate::base::unit
// Provides: {"impl_1087"}
// Dependencies: {}
impl < T : Scalar + simba :: simd :: PrimitiveSimdValue , R : Dim , C : Dim > From < [Unit < OMatrix < T :: Element , R , C > > ; 16] > for Unit < OMatrix < T , R , C > > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 16] > , T :: Element : Scalar , DefaultAllocator : Allocator < R , C > , { # [inline] fn from (arr : [Unit < OMatrix < T :: Element , R , C > > ; 16]) -> Self { Self :: new_unchecked (OMatrix :: from ([arr [0] . clone () . into_inner () , arr [1] . clone () . into_inner () , arr [2] . clone () . into_inner () , arr [3] . clone () . into_inner () , arr [4] . clone () . into_inner () , arr [5] . clone () . into_inner () , arr [6] . clone () . into_inner () , arr [7] . clone () . into_inner () , arr [8] . clone () . into_inner () , arr [9] . clone () . into_inner () , arr [10] . clone () . into_inner () , arr [11] . clone () . into_inner () , arr [12] . clone () . into_inner () , arr [13] . clone () . into_inner () , arr [14] . clone () . into_inner () , arr [15] . clone () . into_inner () ,])) } }
};
}
