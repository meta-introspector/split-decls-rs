// Generated macro for impl_54 (impl)
macro_rules! Depcrate_field25519impl_54 {
() => {
// Module: crate::field25519
// Provides: {"impl_54"}
// Dependencies: {}
impl Mul for Fe { type Output = Fe ; fn mul (self , _rhs : Fe) -> Fe { let Fe (f) = self ; let Fe (g) = _rhs ; let mut h = Fe :: default () ; fiat_25519_carry_mul (& mut h . 0 , & f , & g) ; h } }
};
}
