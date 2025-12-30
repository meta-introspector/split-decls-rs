// Generated macro for impl_100 (impl)
macro_rules! Depcrateimpl_100 {
() => {
// Module: crate
// Provides: {"impl_100"}
// Dependencies: {}
impl < T : Clone + Num > Rem < Complex < T > > for Complex < T > { type Output = Self ; # [inline] fn rem (self , modulus : Self) -> Self :: Output { let gaussian = self . div_trunc (& modulus) ; self - modulus * gaussian } }
};
}
