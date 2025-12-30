// Generated macro for impl_75 (impl)
macro_rules! Depcrateimpl_75 {
() => {
// Module: crate
// Provides: {"impl_75"}
// Dependencies: {}
impl < T : Clone + Num + Neg < Output = T > > Complex < T > { # [doc = " Returns the complex conjugate. i.e. `re - i im`"] # [inline] pub fn conj (& self) -> Self { Self :: new (self . re . clone () , - self . im . clone ()) } # [doc = " Returns `1/self`"] # [inline] pub fn inv (& self) -> Self { let norm_sqr = self . norm_sqr () ; Self :: new (self . re . clone () / norm_sqr . clone () , - self . im . clone () / norm_sqr ,) } # [doc = " Raises `self` to a signed integer power."] # [inline] pub fn powi (& self , exp : i32) -> Self { Pow :: pow (self , exp) } }
};
}
