// Generated macro for camel_case_start (function)
macro_rules! Depcrate_str_utilscamel_case_start {
() => {
// Module: crate::str_utils
// Provides: {"camel_case_start"}
// Dependencies: {}
# [doc = " Returns index of the first camel-case component of `s`."] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use clippy_utils::str_utils::{camel_case_start, StrIndex};"] # [doc = " assert_eq!(camel_case_start(\"AbcDef\"), StrIndex::new(0, 0));"] # [doc = " assert_eq!(camel_case_start(\"abcDef\"), StrIndex::new(3, 3));"] # [doc = " assert_eq!(camel_case_start(\"ABCD\"), StrIndex::new(4, 4));"] # [doc = " assert_eq!(camel_case_start(\"abcd\"), StrIndex::new(4, 4));"] # [doc = " assert_eq!(camel_case_start(\"\\u{f6}\\u{f6}cd\"), StrIndex::new(4, 6));"] # [doc = " ```"] # [must_use] pub fn camel_case_start (s : & str) -> StrIndex { camel_case_start_from_idx (s , 0) }
};
}
