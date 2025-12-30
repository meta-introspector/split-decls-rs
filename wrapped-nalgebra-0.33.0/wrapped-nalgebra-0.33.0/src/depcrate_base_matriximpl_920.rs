// Generated macro for impl_920 (impl)
macro_rules! Depcrate_base_matriximpl_920 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_920"}
// Dependencies: {}
impl < T : SimdComplexField , R : Dim , C : Dim , S : RawStorageMut < T , R , C > > Matrix < T , R , C , S > { # [doc = " The conjugate of the complex matrix `self` computed in-place."] # [inline] pub fn conjugate_mut (& mut self) { self . apply (| e | * e = e . clone () . simd_conjugate ()) } # [doc = " Divides each component of the complex matrix `self` by the given real."] # [inline] pub fn unscale_mut (& mut self , real : T :: SimdRealField) { self . apply (| e | * e = e . clone () . simd_unscale (real . clone ())) } # [doc = " Multiplies each component of the complex matrix `self` by the given real."] # [inline] pub fn scale_mut (& mut self , real : T :: SimdRealField) { self . apply (| e | * e = e . clone () . simd_scale (real . clone ())) } }
};
}
