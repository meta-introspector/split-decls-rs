// Generated macro for split_untagged_check_line (function)
macro_rules! Depcratesplit_untagged_check_line {
() => {
// Module: crate
// Provides: {"split_untagged_check_line"}
// Dependencies: {}
fn split_untagged_check_line (line_after_slash : & str) -> Option < (& str , & str) > { line_after_slash . split_once ("  ") }
};
}
