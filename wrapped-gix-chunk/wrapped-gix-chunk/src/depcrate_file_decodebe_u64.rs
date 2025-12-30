// Generated macro for be_u64 (function)
macro_rules! Depcrate_file_decodebe_u64 {
() => {
// Module: crate::file::decode
// Provides: {"be_u64"}
// Dependencies: {}
fn be_u64 (data : & [u8]) -> u64 { u64 :: from_be_bytes (data [.. 8] . try_into () . unwrap ()) }
};
}
