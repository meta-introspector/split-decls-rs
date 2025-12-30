// Generated macro for impl_465 (impl)
macro_rules! Depcrate_hazardous_ecc_x25519impl_465 {
() => {
// Module: crate::hazardous::ecc::x25519
// Provides: {"impl_465"}
// Dependencies: {}
# [doc = " The function fiat_25519_carry_mul multiplies two field elements and reduces the result."] impl Mul for FieldElement { type Output = Self ; fn mul (self , rhs : Self) -> Self :: Output { let mut ret = fiat_25519_tight_field_element ([0u64 ; 5]) ; let mut self_relaxed = fiat_25519_loose_field_element ([0u64 ; 5]) ; let mut rhs_relaxed = fiat_25519_loose_field_element ([0u64 ; 5]) ; fiat_25519_relax (& mut self_relaxed , & self . 0) ; fiat_25519_relax (& mut rhs_relaxed , & rhs . 0) ; fiat_25519_carry_mul (& mut ret , & self_relaxed , & rhs_relaxed) ; Self (ret) } }
};
}
