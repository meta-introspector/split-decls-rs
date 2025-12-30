// Generated macro for make_file_info (function)
macro_rules! Depcrate_debuginfo_line_infomake_file_info {
() => {
// Module: crate::debuginfo::line_info
// Provides: {"make_file_info"}
// Dependencies: {}
fn make_file_info (hash : SourceFileHash) -> Option < FileInfo > { if hash . kind == SourceFileHashAlgorithm :: Md5 { let mut buf = [0u8 ; MD5_LEN] ; buf . copy_from_slice (hash . hash_bytes ()) ; Some (FileInfo { timestamp : 0 , size : 0 , md5 : buf , source : None , }) } else { None } }
};
}
