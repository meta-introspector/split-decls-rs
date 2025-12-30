// Generated macro for macro_294 (macro)
macro_rules! Depcrate_hazardous_hash_blake2_blake2bmacro_294 {
() => {
// Module: crate::hazardous::hash::blake2::blake2b
// Provides: {"macro_294"}
// Dependencies: {}
construct_public ! { # [doc = " A type to represent the `Digest` that BLAKE2b returns."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is empty."] # [doc = " - `slice` is greater than 64 bytes."] (Digest , test_digest , 1 , BLAKE2B_OUTSIZE) }
};
}
