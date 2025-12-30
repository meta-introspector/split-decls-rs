// Generated macro for impl_94 (impl)
macro_rules! Depcrateimpl_94 {
() => {
// Module: crate
// Provides: {"impl_94"}
// Dependencies: {}
impl < T : Clone + Num + MulAdd < Output = T > > MulAdd < Complex < T > > for Complex < T > { type Output = Complex < T > ; # [inline] fn mul_add (self , other : Complex < T > , add : Complex < T >) -> Complex < T > { let re = self . re . clone () . mul_add (other . re . clone () , add . re) - (self . im . clone () * other . im . clone ()) ; let im = self . re . mul_add (other . im , self . im . mul_add (other . re , add . im)) ; Complex :: new (re , im) } }
};
}
