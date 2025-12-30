// Generated macro for analyze_source_file_dispatch (function)
macro_rules! Depcrateanalyze_source_file_dispatch {
() => {
// Module: crate
// Provides: {"analyze_source_file_dispatch"}
// Dependencies: {}
# [cfg (not (any (target_arch = "x86" , target_arch = "x86_64" , all (target_arch = "aarch64" , target_endian = "little"))))] fn analyze_source_file_dispatch (src : & str , lines : & mut Vec < TextSize > , multi_byte_chars : & mut IntMap < u32 , Vec < WideChar > > ,) { analyze_source_file_generic (src , src . len () , TextSize :: from (0) , lines , multi_byte_chars) ; }
};
}
