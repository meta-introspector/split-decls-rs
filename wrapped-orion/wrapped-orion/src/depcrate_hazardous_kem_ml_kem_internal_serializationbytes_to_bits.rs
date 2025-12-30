// Generated macro for bytes_to_bits (function)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_serializationbytes_to_bits {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::serialization
// Provides: {"bytes_to_bits"}
// Dependencies: {}
# [doc = " FIPS-203, Algorithm 4."] # [doc = " Little-endian order."] pub fn bytes_to_bits (bytes : & [u8] , bits : & mut [u8]) { debug_assert_eq ! (bytes . len () * 8 , bits . len ()) ; for (by , bi) in bytes . iter () . zip (bits . chunks_exact_mut (u8 :: BITS as usize)) { for (idx , exact_bit) in bi . iter_mut () . enumerate () { * exact_bit = (by >> idx) & 1 ; } } debug_assert ! (bits . iter () . all (| x | * x == 0 || * x == 1)) ; }
};
}
