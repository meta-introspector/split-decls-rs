// Generated macro for impl_52 (impl)
macro_rules! Depcrate_field25519impl_52 {
() => {
// Module: crate::field25519
// Provides: {"impl_52"}
// Dependencies: {}
impl Add for Fe { type Output = Fe ; fn add (self , _rhs : Fe) -> Fe { let Fe (f) = self ; let Fe (g) = _rhs ; let mut h = Fe :: default () ; fiat_25519_add (& mut h . 0 , & f , & g) ; h } }
};
}
