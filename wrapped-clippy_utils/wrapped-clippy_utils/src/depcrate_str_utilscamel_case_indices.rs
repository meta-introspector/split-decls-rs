// Generated macro for camel_case_indices (function)
macro_rules! Depcrate_str_utilscamel_case_indices {
() => {
// Module: crate::str_utils
// Provides: {"camel_case_indices"}
// Dependencies: {}
# [doc = " Get the indexes of camel case components of a string `s`"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use clippy_utils::str_utils::{camel_case_indices, StrIndex};"] # [doc = " assert_eq!("] # [doc = "     camel_case_indices(\"AbcDef\"),"] # [doc = "     vec![StrIndex::new(0, 0), StrIndex::new(3, 3), StrIndex::new(6, 6)]"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     camel_case_indices(\"abcDef\"),"] # [doc = "     vec![StrIndex::new(3, 3), StrIndex::new(6, 6)]"] # [doc = " );"] # [doc = " ```"] pub fn camel_case_indices (s : & str) -> Vec < StrIndex > { let mut result = Vec :: new () ; let mut str_idx = camel_case_start (s) ; while str_idx . byte_index < s . len () { let next_idx = str_idx . byte_index + 1 ; result . push (str_idx) ; str_idx = camel_case_start_from_idx (s , next_idx) ; } result . push (str_idx) ; result }
};
}
