// Generated macro for camel_case_split (function)
macro_rules! Depcrate_str_utilscamel_case_split {
() => {
// Module: crate::str_utils
// Provides: {"camel_case_split"}
// Dependencies: {}
# [doc = " Split camel case string into a vector of its components"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use clippy_utils::str_utils::{camel_case_split, StrIndex};"] # [doc = " assert_eq!(camel_case_split(\"AbcDef\"), vec![\"Abc\", \"Def\"]);"] # [doc = " ```"] pub fn camel_case_split (s : & str) -> Vec < & str > { let mut offsets = camel_case_indices (s) . iter () . map (| e | e . byte_index) . collect :: < Vec < usize > > () ; if offsets [0] != 0 { offsets . insert (0 , 0) ; } offsets . windows (2) . map (| w | & s [w [0] .. w [1]]) . collect () }
};
}
