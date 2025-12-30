// Generated macro for impl_1027 (impl)
macro_rules! Depcrate_base_normimpl_1027 {
() => {
// Module: crate::base::norm
// Provides: {"impl_1027"}
// Dependencies: {}
# [doc = " # In-place normalization"] impl < T : Scalar , R : Dim , C : Dim , S : StorageMut < T , R , C > > Matrix < T , R , C , S > { # [doc = " Normalizes this matrix in-place and returns its norm."] # [doc = ""] # [doc = " The components of the matrix cannot be SIMD types (see `simd_try_normalize_mut` instead)."] # [inline] pub fn normalize_mut (& mut self) -> T :: SimdRealField where T : SimdComplexField , { let n = self . norm () ; self . unscale_mut (n . clone ()) ; n } # [doc = " Normalizes this matrix in-place and return its norm."] # [doc = ""] # [doc = " The components of the matrix can be SIMD types."] # [inline] # [must_use = "Did you mean to use simd_try_normalize_mut()?"] pub fn simd_try_normalize_mut (& mut self , min_norm : T :: SimdRealField ,) -> SimdOption < T :: SimdRealField > where T : SimdComplexField , T :: Element : Scalar , DefaultAllocator : Allocator < R , C > , { let n = self . norm () ; let le = n . clone () . simd_le (min_norm) ; self . apply (| e | * e = e . clone () . simd_unscale (n . clone ()) . select (le , e . clone ())) ; SimdOption :: new (n , le) } # [doc = " Normalizes this matrix in-place or does nothing if its norm is smaller or equal to `eps`."] # [doc = ""] # [doc = " If the normalization succeeded, returns the old norm of this matrix."] # [inline] pub fn try_normalize_mut (& mut self , min_norm : T :: RealField) -> Option < T :: RealField > where T : ComplexField , { let n = self . norm () ; if n <= min_norm { None } else { self . unscale_mut (n . clone ()) ; Some (n) } } }
};
}
