// Generated macro for g (function)
macro_rules! Depcrate_hazardous_kem_ml_kem_internalg {
() => {
// Module: crate::hazardous::kem::ml_kem::internal
// Provides: {"g"}
// Dependencies: {}
# [doc = " FIPS-203, Def. 4.5"] pub fn g (c : & [& [u8]]) -> ([u8 ; 32] , Zeroizing < [u8 ; 32] >) { let mut state = Sha3_512 :: new () ; for input in c . iter () { state . update (input) . unwrap () ; } let hash = state . finalize () . unwrap () ; let mut rho = [0u8 ; 32] ; let mut sigma = Zeroizing :: new ([0u8 ; 32]) ; rho . copy_from_slice (& hash . as_ref () [0 .. 32]) ; sigma . copy_from_slice (& hash . as_ref () [32 .. 64]) ; (rho , sigma) }
};
}
