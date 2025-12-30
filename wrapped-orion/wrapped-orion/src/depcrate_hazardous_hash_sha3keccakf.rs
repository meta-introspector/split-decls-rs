// Generated macro for keccakf (function)
macro_rules! Depcrate_hazardous_hash_sha3keccakf {
() => {
// Module: crate::hazardous::hash::sha3
// Provides: {"keccakf"}
// Dependencies: {}
fn keccakf < const ROUNDS : usize > (state : & mut [u64 ; 25]) { for round in 0 .. ROUNDS { let mut buf = [0u64 ; 5] ; theta (state , & mut buf) ; rho_and_pi (state , & mut buf) ; chi (state , & mut buf) ; iota (state , round) ; } }
};
}
