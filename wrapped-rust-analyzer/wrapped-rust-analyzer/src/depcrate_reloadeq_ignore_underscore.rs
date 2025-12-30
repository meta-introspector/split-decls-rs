// Generated macro for eq_ignore_underscore (function)
macro_rules! Depcrate_reloadeq_ignore_underscore {
() => {
// Module: crate::reload
// Provides: {"eq_ignore_underscore"}
// Dependencies: {}
# [doc = " Similar to [`str::eq_ignore_ascii_case`] but instead of ignoring"] # [doc = " case, we say that `-` and `_` are equal."] fn eq_ignore_underscore (s1 : & str , s2 : & str) -> bool { if s1 . len () != s2 . len () { return false ; } s1 . as_bytes () . iter () . zip (s2 . as_bytes ()) . all (| (c1 , c2) | { let c1_underscore = c1 == & b'_' || c1 == & b'-' ; let c2_underscore = c2 == & b'_' || c2 == & b'-' ; c1 == c2 || (c1_underscore && c2_underscore) }) }
};
}
