// Generated macro for impl_1086 (impl)
macro_rules! Depcrate_base_unitimpl_1086 {
() => {
// Module: crate::base::unit
// Provides: {"impl_1086"}
// Dependencies: {}
impl < T : Scalar + simba :: simd :: PrimitiveSimdValue , R : Dim , C : Dim > From < [Unit < OMatrix < T :: Element , R , C > > ; 8] > for Unit < OMatrix < T , R , C > > where T : From < [< T as simba :: simd :: SimdValue > :: Element ; 8] > , T :: Element : Scalar , DefaultAllocator : Allocator < R , C > , { # [inline] fn from (arr : [Unit < OMatrix < T :: Element , R , C > > ; 8]) -> Self { Self :: new_unchecked (OMatrix :: from ([arr [0] . clone () . into_inner () , arr [1] . clone () . into_inner () , arr [2] . clone () . into_inner () , arr [3] . clone () . into_inner () , arr [4] . clone () . into_inner () , arr [5] . clone () . into_inner () , arr [6] . clone () . into_inner () , arr [7] . clone () . into_inner () ,])) } }
};
}
