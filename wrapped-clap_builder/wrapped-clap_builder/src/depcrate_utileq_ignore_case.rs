// Generated macro for eq_ignore_case (function)
macro_rules! Depcrate_utileq_ignore_case {
() => {
// Module: crate::util
// Provides: {"eq_ignore_case"}
// Dependencies: {}
# [cfg (not (feature = "unicode"))] pub (crate) fn eq_ignore_case (left : & str , right : & str) -> bool { left . eq_ignore_ascii_case (right) }
};
}
