// Generated macro for Shake (struct)
macro_rules! Depcrate_hazardous_hash_sha3Shake {
() => {
// Module: crate::hazardous::hash::sha3
// Provides: {"Shake"}
// Dependencies: {}
# [derive (Clone)] # [doc = " SHAKE streaming state."] pub (crate) struct Shake < const RATE : usize > { pub (crate) state : [u64 ; 25] , pub (crate) buffer : [u8 ; RATE] , pub (crate) capacity : usize , until_absorb : usize , to_squeeze : usize , is_finalized : bool , }
};
}
