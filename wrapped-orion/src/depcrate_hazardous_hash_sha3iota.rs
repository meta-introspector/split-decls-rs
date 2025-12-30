// Generated macro for iota (function)
macro_rules! Depcrate_hazardous_hash_sha3iota {
() => {
// Module: crate::hazardous::hash::sha3
// Provides: {"iota"}
// Dependencies: {}
fn iota (state : & mut [u64 ; 25] , round : usize) { debug_assert ! (round <= 24) ; state [0] ^= RC [round] ; }
};
}
