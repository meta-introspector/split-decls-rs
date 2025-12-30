// Generated macro for cmp_ignore_ascii_case (function)
macro_rules! Depcrate_util_utf8cmp_ignore_ascii_case {
() => {
// Module: crate::util::utf8
// Provides: {"cmp_ignore_ascii_case"}
// Dependencies: {}
# [doc = " Like std's `eq_ignore_ascii_case`, but returns a full `Ordering`."] # [inline] pub (crate) fn cmp_ignore_ascii_case (s1 : & str , s2 : & str) -> Ordering { cmp_ignore_ascii_case_bytes (s1 . as_bytes () , s2 . as_bytes ()) }
};
}
