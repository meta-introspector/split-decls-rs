// Generated macro for extract_cdb_version (function)
macro_rules! Depcrate_debuggersextract_cdb_version {
() => {
// Module: crate::debuggers
// Provides: {"extract_cdb_version"}
// Dependencies: {}
pub (crate) fn extract_cdb_version (full_version_line : & str) -> Option < [u16 ; 4] > { let version = full_version_line . rsplit (' ') . next () ? ; let mut components = version . split ('.') ; let major : u16 = components . next () . unwrap () . parse () . unwrap () ; let minor : u16 = components . next () . unwrap () . parse () . unwrap () ; let patch : u16 = components . next () . unwrap_or ("0") . parse () . unwrap () ; let build : u16 = components . next () . unwrap_or ("0") . parse () . unwrap () ; Some ([major , minor , patch , build]) }
};
}
