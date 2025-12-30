// Generated macro for mz_crc32_oxide (function)
macro_rules! Depcrate_c_exportmz_crc32_oxide {
() => {
// Module: crate::c_export
// Provides: {"mz_crc32_oxide"}
// Dependencies: {}
pub fn mz_crc32_oxide (crc32 : c_uint , data : & [u8]) -> c_uint { let mut digest = crc32fast :: Hasher :: new_with_initial (crc32) ; digest . update (data) ; digest . finalize () }
};
}
