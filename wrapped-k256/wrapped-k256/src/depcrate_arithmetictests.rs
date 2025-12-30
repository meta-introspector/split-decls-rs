// Generated macro for tests (module)
macro_rules! Depcrate_arithmetictests {
() => {
// Module: crate::arithmetic
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: CURVE_EQUATION_B ; use hex_literal :: hex ; const CURVE_EQUATION_B_BYTES : [u8 ; 32] = hex ! ("0000000000000000000000000000000000000000000000000000000000000007") ; # [test] fn verify_constants () { assert_eq ! (CURVE_EQUATION_B . to_bytes () , CURVE_EQUATION_B_BYTES) ; } # [test] fn try_from_rng () { use crate :: SecretKey ; use rand :: rngs :: OsRng ; let key = SecretKey :: try_from_rng (& mut OsRng) . unwrap () ; assert ! (! key . to_bytes () . iter () . all (| b | * b == 0)) } }
};
}
