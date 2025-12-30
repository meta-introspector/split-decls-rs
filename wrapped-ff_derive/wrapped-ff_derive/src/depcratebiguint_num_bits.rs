// Generated macro for biguint_num_bits (function)
macro_rules! Depcratebiguint_num_bits {
() => {
// Module: crate
// Provides: {"biguint_num_bits"}
// Dependencies: {}
fn biguint_num_bits (mut v : BigUint) -> u32 { let mut bits = 0 ; while v != BigUint :: zero () { v >>= 1 ; bits += 1 ; } bits }
};
}
