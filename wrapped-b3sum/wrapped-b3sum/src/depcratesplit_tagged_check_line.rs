// Generated macro for split_tagged_check_line (function)
macro_rules! Depcratesplit_tagged_check_line {
() => {
// Module: crate
// Provides: {"split_tagged_check_line"}
// Dependencies: {}
fn split_tagged_check_line (line_after_slash : & str) -> Option < (& str , & str) > { let prefix = "BLAKE3 (" ; if ! line_after_slash . starts_with (prefix) { return None ; } line_after_slash [prefix . len () ..] . rsplit_once (") = ") }
};
}
