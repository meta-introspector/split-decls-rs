// Generated macro for parse_version (function)
macro_rules! Depcrate_versionparse_version {
() => {
// Module: crate::version
// Provides: {"parse_version"}
// Dependencies: {}
fn parse_version (s : & str) -> Option < (u64 , u64 , u64) > { let mut iter = s . trim () . split_terminator ('.') . fuse () ; let major = iter . next () . and_then (| s | s . parse () . ok ()) ? ; let minor = iter . next () . unwrap_or ("0") . parse () . ok () ? ; let patch = iter . next () . unwrap_or ("0") . parse () . ok () ? ; if iter . next () . is_some () { return None ; } Some ((major , minor , patch)) }
};
}
