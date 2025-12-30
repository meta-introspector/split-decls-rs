// Generated macro for impl_945 (impl)
macro_rules! Depcrate_base_matriximpl_945 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_945"}
// Dependencies: {}
impl < T : SimdComplexField , R : Dim , C : Dim , S : Storage < T , R , C > > Matrix < T , R , C , S > { # [doc = " The smallest angle between two vectors."] # [inline] # [must_use] pub fn angle < R2 : Dim , C2 : Dim , SB > (& self , other : & Matrix < T , R2 , C2 , SB >) -> T :: SimdRealField where SB : Storage < T , R2 , C2 > , ShapeConstraint : DimEq < R , R2 > + DimEq < C , C2 > , { let prod = self . dotc (other) ; let n1 = self . norm () ; let n2 = other . norm () ; if n1 . is_zero () || n2 . is_zero () { T :: SimdRealField :: zero () } else { let cang = prod . simd_real () / (n1 * n2) ; cang . simd_clamp (- T :: SimdRealField :: one () , T :: SimdRealField :: one ()) . simd_acos () } } }
};
}
