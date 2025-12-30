// Generated macro for extract_lldb_version (function)
macro_rules! Depcrate_debuggersextract_lldb_version {
() => {
// Module: crate::debuggers
// Provides: {"extract_lldb_version"}
// Dependencies: {}
# [doc = " Returns LLDB version"] pub (crate) fn extract_lldb_version (full_version_line : & str) -> Option < u32 > { let full_version_line = full_version_line . trim () ; if let Some (apple_ver) = full_version_line . strip_prefix ("LLDB-") . or_else (| | full_version_line . strip_prefix ("lldb-")) { if let Some (idx) = apple_ver . find (not_a_digit) { let version : u32 = apple_ver [.. idx] . parse () . unwrap () ; return Some (version) ; } } else if let Some (lldb_ver) = full_version_line . strip_prefix ("lldb version ") { if let Some (idx) = lldb_ver . find (not_a_digit) { let version : u32 = lldb_ver [.. idx] . parse () . ok () ? ; return Some (version * 100) ; } } None }
};
}
