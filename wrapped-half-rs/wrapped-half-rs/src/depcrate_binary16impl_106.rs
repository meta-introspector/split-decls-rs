// Generated macro for impl_106 (impl)
macro_rules! Depcrate_binary16impl_106 {
() => {
// Module: crate::binary16
// Provides: {"impl_106"}
// Dependencies: {}
impl Mul for f16 { type Output = Self ; # [inline] fn mul (self , rhs : Self) -> Self :: Output { f16 (arch :: multiply_f16 (self . 0 , rhs . 0)) } }
};
}
