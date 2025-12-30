// Generated macro for impl_53 (impl)
macro_rules! Depcrate_field25519impl_53 {
() => {
// Module: crate::field25519
// Provides: {"impl_53"}
// Dependencies: {}
impl Sub for Fe { type Output = Fe ; fn sub (self , _rhs : Fe) -> Fe { let Fe (f) = self ; let Fe (g) = _rhs ; let mut h = Fe :: default () ; fiat_25519_sub (& mut h . 0 , & f , & g) ; h . carry () } }
};
}
