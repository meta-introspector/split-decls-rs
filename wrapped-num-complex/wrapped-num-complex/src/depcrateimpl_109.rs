// Generated macro for impl_109 (impl)
macro_rules! Depcrateimpl_109 {
() => {
// Module: crate
// Provides: {"impl_109"}
// Dependencies: {}
impl < T : Clone + Num > Mul < T > for Complex < T > { type Output = Complex < T > ; # [inline] fn mul (self , other : T) -> Self :: Output { Self :: Output :: new (self . re * other . clone () , self . im * other) } }
};
}
