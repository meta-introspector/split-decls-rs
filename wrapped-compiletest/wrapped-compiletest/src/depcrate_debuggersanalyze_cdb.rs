// Generated macro for analyze_cdb (function)
macro_rules! Depcrate_debuggersanalyze_cdb {
() => {
// Module: crate::debuggers
// Provides: {"analyze_cdb"}
// Dependencies: {}
# [doc = " Returns Path to CDB"] pub (crate) fn analyze_cdb (cdb : Option < String > , target : & str ,) -> (Option < Utf8PathBuf > , Option < [u16 ; 4] >) { let cdb = cdb . map (Utf8PathBuf :: from) . or_else (| | find_cdb (target)) ; let mut version = None ; if let Some (cdb) = cdb . as_ref () { if let Ok (output) = Command :: new (cdb) . arg ("/version") . output () { if let Some (first_line) = String :: from_utf8_lossy (& output . stdout) . lines () . next () { version = extract_cdb_version (& first_line) ; } } } (cdb , version) }
};
}
