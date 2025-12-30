// Generated macro for Norm (trait)
macro_rules! Depcrate_base_normNorm {
() => {
// Module: crate::base::norm
// Provides: {"Norm"}
// Dependencies: {}
# [doc = " A trait for abstract matrix norms."] # [doc = ""] # [doc = " This may be moved to the alga crate in the future."] pub trait Norm < T : SimdComplexField > { # [doc = " Apply this norm to the given matrix."] fn norm < R , C , S > (& self , m : & Matrix < T , R , C , S >) -> T :: SimdRealField where R : Dim , C : Dim , S : Storage < T , R , C > ; # [doc = " Use the metric induced by this norm to compute the metric distance between the two given matrices."] fn metric_distance < R1 , C1 , S1 , R2 , C2 , S2 > (& self , m1 : & Matrix < T , R1 , C1 , S1 > , m2 : & Matrix < T , R2 , C2 , S2 > ,) -> T :: SimdRealField where R1 : Dim , C1 : Dim , S1 : Storage < T , R1 , C1 > , R2 : Dim , C2 : Dim , S2 : Storage < T , R2 , C2 > , ShapeConstraint : SameNumberOfRows < R1 , R2 > + SameNumberOfColumns < C1 , C2 > ; }
};
}
