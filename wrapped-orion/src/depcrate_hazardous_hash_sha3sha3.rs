// Generated macro for Sha3 (struct)
macro_rules! Depcrate_hazardous_hash_sha3Sha3 {
() => {
// Module: crate::hazardous::hash::sha3
// Provides: {"Sha3"}
// Dependencies: {}
# [derive (Clone)] # [doc = " SHA3 streaming state."] pub (crate) struct Sha3 < const RATE : usize > { pub (crate) state : [u64 ; 25] , pub (crate) buffer : [u8 ; RATE] , pub (crate) capacity : usize , leftover : usize , is_finalized : bool , }
};
}
