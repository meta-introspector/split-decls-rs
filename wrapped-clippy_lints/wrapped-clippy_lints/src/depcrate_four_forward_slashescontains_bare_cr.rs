// Generated macro for contains_bare_cr (function)
macro_rules! Depcrate_four_forward_slashescontains_bare_cr {
() => {
// Module: crate::four_forward_slashes
// Provides: {"contains_bare_cr"}
// Dependencies: {}
# [doc = " Checks if `text` contains any CR not followed by a LF"] fn contains_bare_cr (text : & str) -> bool { text . bytes () . tuple_windows () . any (| (a , b) | a == b'\r' && b != b'\n') }
};
}
