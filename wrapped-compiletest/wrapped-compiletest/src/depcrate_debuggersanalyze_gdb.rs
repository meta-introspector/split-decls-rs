// Generated macro for analyze_gdb (function)
macro_rules! Depcrate_debuggersanalyze_gdb {
() => {
// Module: crate::debuggers
// Provides: {"analyze_gdb"}
// Dependencies: {}
# [doc = " Returns (Path to GDB, GDB Version)"] pub (crate) fn analyze_gdb (gdb : Option < String > , target : & str , android_cross_path : & Utf8Path ,) -> (Option < String > , Option < u32 >) { # [cfg (not (windows))] const GDB_FALLBACK : & str = "gdb" ; # [cfg (windows)] const GDB_FALLBACK : & str = "gdb.exe" ; let fallback_gdb = | | { if is_android_gdb_target (target) { let mut gdb_path = android_cross_path . to_string () ; gdb_path . push_str ("/bin/gdb") ; gdb_path } else { GDB_FALLBACK . to_owned () } } ; let gdb = match gdb { None => fallback_gdb () , Some (ref s) if s . is_empty () => fallback_gdb () , Some (ref s) => s . to_owned () , } ; let mut version_line = None ; if let Ok (output) = Command :: new (& gdb) . arg ("--version") . output () { if let Some (first_line) = String :: from_utf8_lossy (& output . stdout) . lines () . next () { version_line = Some (first_line . to_string ()) ; } } let version = match version_line { Some (line) => extract_gdb_version (& line) , None => return (None , None) , } ; (Some (gdb) , version) }
};
}
