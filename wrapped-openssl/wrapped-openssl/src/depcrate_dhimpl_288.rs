// Generated macro for impl_288 (impl)
macro_rules! Depcrate_dhimpl_288 {
() => {
// Module: crate::dh
// Provides: {"impl_288"}
// Dependencies: {}
impl < T > Dh < T > where T : HasParams , { # [doc = " Returns the prime `p` from the DH instance."] # [corresponds (DH_get0_pqg)] pub fn prime_p (& self) -> & BigNumRef { let mut p = ptr :: null () ; unsafe { DH_get0_pqg (self . as_ptr () , & mut p , ptr :: null_mut () , ptr :: null_mut ()) ; BigNumRef :: from_ptr (p as * mut _) } } # [doc = " Returns the prime `q` from the DH instance."] # [corresponds (DH_get0_pqg)] pub fn prime_q (& self) -> Option < & BigNumRef > { let mut q = ptr :: null () ; unsafe { DH_get0_pqg (self . as_ptr () , ptr :: null_mut () , & mut q , ptr :: null_mut ()) ; if q . is_null () { None } else { Some (BigNumRef :: from_ptr (q as * mut _)) } } } # [doc = " Returns the generator from the DH instance."] # [corresponds (DH_get0_pqg)] pub fn generator (& self) -> & BigNumRef { let mut g = ptr :: null () ; unsafe { DH_get0_pqg (self . as_ptr () , ptr :: null_mut () , ptr :: null_mut () , & mut g) ; BigNumRef :: from_ptr (g as * mut _) } } }
};
}
