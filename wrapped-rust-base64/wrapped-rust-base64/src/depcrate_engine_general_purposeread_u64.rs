// Generated macro for read_u64 (function)
macro_rules! Depcrate_engine_general_purposeread_u64 {
() => {
// Module: crate::engine::general_purpose
// Provides: {"read_u64"}
// Dependencies: {}
# [inline] fn read_u64 (s : & [u8]) -> u64 { u64 :: from_be_bytes (s [.. 8] . try_into () . unwrap ()) }
};
}
