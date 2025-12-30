// Generated macro for impl_467 (impl)
macro_rules! Depcrate_hazardous_ecc_x25519impl_467 {
() => {
// Module: crate::hazardous::ecc::x25519
// Provides: {"impl_467"}
// Dependencies: {}
# [doc = " The function fiat_25519_sub subtracts two field elements."] impl Sub for FieldElement { type Output = Self ; fn sub (self , rhs : Self) -> Self :: Output { let mut ret = fiat_25519_tight_field_element ([0u64 ; 5]) ; let mut ret_sub = fiat_25519_loose_field_element ([0u64 ; 5]) ; fiat_25519_sub (& mut ret_sub , & self . 0 , & rhs . 0) ; fiat_25519_carry (& mut ret , & ret_sub) ; Self (ret) } }
};
}
