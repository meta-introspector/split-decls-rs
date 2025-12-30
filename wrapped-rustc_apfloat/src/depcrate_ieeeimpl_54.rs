// Generated macro for impl_54 (impl)
macro_rules! Depcrate_ieeeimpl_54 {
() => {
// Module: crate::ieee
// Provides: {"impl_54"}
// Dependencies: {}
impl < S : Semantics > Neg for IeeeFloat < S > { type Output = Self ; fn neg (mut self) -> Self { self . read_only_sign_do_not_mutate = ! self . is_negative () ; self } }
};
}
