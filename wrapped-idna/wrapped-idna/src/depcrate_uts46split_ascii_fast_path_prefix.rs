// Generated macro for split_ascii_fast_path_prefix (function)
macro_rules! Depcrate_uts46split_ascii_fast_path_prefix {
() => {
// Module: crate::uts46
// Provides: {"split_ascii_fast_path_prefix"}
// Dependencies: {}
# [inline (always)] fn split_ascii_fast_path_prefix (label : & [u8]) -> (& [u8] , & [u8]) { if let Some (pos) = label . iter () . position (| b | ! b . is_ascii ()) { if pos == 0 { (& [] , label) } else { let (head , tail) = label . split_at (pos - 1) ; (head , tail) } } else { (label , & []) } }
};
}
