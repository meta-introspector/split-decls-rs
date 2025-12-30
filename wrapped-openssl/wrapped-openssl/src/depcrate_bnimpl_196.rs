// Generated macro for impl_196 (impl)
macro_rules! Depcrate_bnimpl_196 {
() => {
// Module: crate::bn
// Provides: {"impl_196"}
// Dependencies: {}
impl Neg for BigNum { type Output = BigNum ; fn neg (mut self) -> BigNum { let negative = self . is_negative () ; self . set_negative (! negative) ; self } }
};
}
