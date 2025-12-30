// Generated macro for impl_921 (impl)
macro_rules! Depcrate_base_matriximpl_921 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_921"}
// Dependencies: {}
impl < T : SimdComplexField , D : Dim , S : RawStorageMut < T , D , D > > Matrix < T , D , D , S > { # [doc = " Sets `self` to its adjoint."] # [deprecated (note = "Renamed to `self.adjoint_mut()`.")] pub fn conjugate_transform_mut (& mut self) { self . adjoint_mut () } # [doc = " Sets `self` to its adjoint (aka. conjugate-transpose)."] pub fn adjoint_mut (& mut self) { assert ! (self . is_square () , "Unable to transpose a non-square matrix in-place.") ; let dim = self . shape () . 0 ; for i in 0 .. dim { for j in 0 .. i { unsafe { let ref_ij = self . get_unchecked ((i , j)) . clone () ; let ref_ji = self . get_unchecked ((j , i)) . clone () ; let conj_ij = ref_ij . simd_conjugate () ; let conj_ji = ref_ji . simd_conjugate () ; * self . get_unchecked_mut ((i , j)) = conj_ji ; * self . get_unchecked_mut ((j , i)) = conj_ij ; } } { let diag = unsafe { self . get_unchecked_mut ((i , i)) } ; * diag = diag . clone () . simd_conjugate () ; } } } }
};
}
