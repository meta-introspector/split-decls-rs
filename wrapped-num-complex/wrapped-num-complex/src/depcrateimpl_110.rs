// Generated macro for impl_110 (impl)
macro_rules! Depcrateimpl_110 {
() => {
// Module: crate
// Provides: {"impl_110"}
// Dependencies: {}
impl < T : Clone + Num > Div < T > for Complex < T > { type Output = Self ; # [inline] fn div (self , other : T) -> Self :: Output { Self :: Output :: new (self . re / other . clone () , self . im / other) } }
};
}
