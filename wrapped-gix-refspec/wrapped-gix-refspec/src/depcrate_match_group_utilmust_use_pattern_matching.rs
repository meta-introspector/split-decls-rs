// Generated macro for must_use_pattern_matching (function)
macro_rules! Depcrate_match_group_utilmust_use_pattern_matching {
() => {
// Module: crate::match_group::util
// Provides: {"must_use_pattern_matching"}
// Dependencies: {}
# [doc = " Check if a pattern is complex enough to require wildmatch instead of simple glob matching"] fn must_use_pattern_matching (pattern : & BStr) -> bool { let asterisk_count = pattern . iter () . filter (| & & b | b == b'*') . count () ; if asterisk_count > 1 { return true ; } pattern . iter () . any (| & b | b == b'?' || b == b'[' || b == b']' || b == b'\\') }
};
}
