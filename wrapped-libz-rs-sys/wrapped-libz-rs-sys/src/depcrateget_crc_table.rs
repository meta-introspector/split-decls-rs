// Generated macro for get_crc_table (function)
macro_rules! Depcrateget_crc_table {
() => {
// Module: crate
// Provides: {"get_crc_table"}
// Dependencies: {}
# [doc = " The CRC table used by the crc32 checksum algorithm."] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (get_crc_table))] pub extern "C" fn get_crc_table () -> * const [u32 ; 256] { zlib_rs :: get_crc_table () }
};
}
