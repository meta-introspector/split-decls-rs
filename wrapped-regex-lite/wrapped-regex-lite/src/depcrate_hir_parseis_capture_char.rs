// Generated macro for is_capture_char (function)
macro_rules! Depcrate_hir_parseis_capture_char {
() => {
// Module: crate::hir::parse
// Provides: {"is_capture_char"}
// Dependencies: {}
# [doc = " Returns true if the given character is a valid in a capture group name."] # [doc = ""] # [doc = " If `first` is true, then `c` is treated as the first character in the"] # [doc = " group name (which must be alphabetic or underscore)."] fn is_capture_char (c : char , first : bool) -> bool { if first { c == '_' || c . is_alphabetic () } else { c == '_' || c == '.' || c == '[' || c == ']' || c . is_alphanumeric () } }
};
}
