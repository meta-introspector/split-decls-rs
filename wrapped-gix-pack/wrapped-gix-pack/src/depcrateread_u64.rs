// Generated macro for read_u64 (function)
macro_rules! Depcrateread_u64 {
() => {
// Module: crate
// Provides: {"read_u64"}
// Dependencies: {}
# [inline] fn read_u64 (b : & [u8]) -> u64 { u64 :: from_be_bytes (b . try_into () . unwrap ()) }
};
}
