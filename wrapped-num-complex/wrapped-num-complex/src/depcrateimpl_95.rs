// Generated macro for impl_95 (impl)
macro_rules! Depcrateimpl_95 {
() => {
// Module: crate
// Provides: {"impl_95"}
// Dependencies: {}
impl < 'a , 'b , T : Clone + Num + MulAdd < Output = T > > MulAdd < & 'b Complex < T > > for & 'a Complex < T > { type Output = Complex < T > ; # [inline] fn mul_add (self , other : & Complex < T > , add : & Complex < T >) -> Complex < T > { self . clone () . mul_add (other . clone () , add . clone ()) } }
};
}
