// Generated macro for extract_gdb_version (function)
macro_rules! Depcrate_debuggersextract_gdb_version {
() => {
// Module: crate::debuggers
// Provides: {"extract_gdb_version"}
// Dependencies: {}
pub (crate) fn extract_gdb_version (full_version_line : & str) -> Option < u32 > { let full_version_line = full_version_line . trim () ; let unbracketed_part = full_version_line . split ('[') . next () . unwrap () ; let mut splits = unbracketed_part . trim_end () . rsplit (' ') ; let version_string = splits . next () . unwrap () ; let mut splits = version_string . split ('.') ; let major = splits . next () . unwrap () ; let minor = splits . next () . unwrap () ; let patch = splits . next () ; let major : u32 = major . parse () . unwrap () ; let (minor , patch) : (u32 , u32) = match minor . find (not_a_digit) { None => { let minor = minor . parse () . unwrap () ; let patch : u32 = match patch { Some (patch) => match patch . find (not_a_digit) { None => patch . parse () . unwrap () , Some (idx) if idx > 3 => 0 , Some (idx) => patch [.. idx] . parse () . unwrap () , } , None => 0 , } ; (minor , patch) } Some (idx) => { let minor = minor [.. idx] . parse () . unwrap () ; (minor , 0) } } ; Some (((major * 1000) + minor) * 1000 + patch) }
};
}
