// Generated macro for read_u32 (function)
macro_rules! Depcrate_file_commitread_u32 {
() => {
// Module: crate::file::commit
// Provides: {"read_u32"}
// Dependencies: {}
# [inline] fn read_u32 (b : & [u8]) -> u32 { u32 :: from_be_bytes (b . try_into () . unwrap ()) }
};
}
