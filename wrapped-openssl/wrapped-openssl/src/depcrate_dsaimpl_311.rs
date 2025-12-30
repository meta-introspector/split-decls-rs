// Generated macro for impl_311 (impl)
macro_rules! Depcrate_dsaimpl_311 {
() => {
// Module: crate::dsa
// Provides: {"impl_311"}
// Dependencies: {}
impl < T > DsaRef < T > where T : HasParams , { # [doc = " Returns the maximum size of the signature output by `self` in bytes."] # [corresponds (DSA_size)] pub fn size (& self) -> u32 { unsafe { ffi :: DSA_size (self . as_ptr ()) as u32 } } # [doc = " Returns the DSA prime parameter of `self`."] # [corresponds (DSA_get0_pqg)] pub fn p (& self) -> & BigNumRef { unsafe { let mut p = ptr :: null () ; DSA_get0_pqg (self . as_ptr () , & mut p , ptr :: null_mut () , ptr :: null_mut ()) ; BigNumRef :: from_const_ptr (p) } } # [doc = " Returns the DSA sub-prime parameter of `self`."] # [corresponds (DSA_get0_pqg)] pub fn q (& self) -> & BigNumRef { unsafe { let mut q = ptr :: null () ; DSA_get0_pqg (self . as_ptr () , ptr :: null_mut () , & mut q , ptr :: null_mut ()) ; BigNumRef :: from_const_ptr (q) } } # [doc = " Returns the DSA base parameter of `self`."] # [corresponds (DSA_get0_pqg)] pub fn g (& self) -> & BigNumRef { unsafe { let mut g = ptr :: null () ; DSA_get0_pqg (self . as_ptr () , ptr :: null_mut () , ptr :: null_mut () , & mut g) ; BigNumRef :: from_const_ptr (g) } } }
};
}
