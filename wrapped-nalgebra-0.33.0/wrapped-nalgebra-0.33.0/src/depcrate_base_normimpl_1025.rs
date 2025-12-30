// Generated macro for impl_1025 (impl)
macro_rules! Depcrate_base_normimpl_1025 {
() => {
// Module: crate::base::norm
// Provides: {"impl_1025"}
// Dependencies: {}
impl < T : SimdComplexField > Norm < T > for UniformNorm { # [inline] fn norm < R , C , S > (& self , m : & Matrix < T , R , C , S >) -> T :: SimdRealField where R : Dim , C : Dim , S : Storage < T , R , C > , { m . fold (T :: SimdRealField :: zero () , | acc , a | { acc . simd_max (a . simd_modulus ()) }) } # [inline] fn metric_distance < R1 , C1 , S1 , R2 , C2 , S2 > (& self , m1 : & Matrix < T , R1 , C1 , S1 > , m2 : & Matrix < T , R2 , C2 , S2 > ,) -> T :: SimdRealField where R1 : Dim , C1 : Dim , S1 : Storage < T , R1 , C1 > , R2 : Dim , C2 : Dim , S2 : Storage < T , R2 , C2 > , ShapeConstraint : SameNumberOfRows < R1 , R2 > + SameNumberOfColumns < C1 , C2 > , { m1 . zip_fold (m2 , T :: SimdRealField :: zero () , | acc , a , b | { let val = (a - b) . simd_modulus () ; acc . simd_max (val) }) } }
};
}
