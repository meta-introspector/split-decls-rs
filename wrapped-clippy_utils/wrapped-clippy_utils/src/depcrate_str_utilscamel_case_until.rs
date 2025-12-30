// Generated macro for camel_case_until (function)
macro_rules! Depcrate_str_utilscamel_case_until {
() => {
// Module: crate::str_utils
// Provides: {"camel_case_until"}
// Dependencies: {}
# [doc = " Returns the index of the character after the first camel-case component of `s`."] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use clippy_utils::str_utils::{camel_case_until, StrIndex};"] # [doc = " assert_eq!(camel_case_until(\"AbcDef\"), StrIndex::new(6, 6));"] # [doc = " assert_eq!(camel_case_until(\"ABCD\"), StrIndex::new(0, 0));"] # [doc = " assert_eq!(camel_case_until(\"AbcDD\"), StrIndex::new(3, 3));"] # [doc = " assert_eq!(camel_case_until(\"Abc\\u{f6}\\u{f6}DD\"), StrIndex::new(5, 7));"] # [doc = " ```"] # [must_use] pub fn camel_case_until (s : & str) -> StrIndex { let mut iter = s . char_indices () . enumerate () ; if let Some ((_char_index , (_ , first))) = iter . next () { if ! first . is_uppercase () { return StrIndex :: new (0 , 0) ; } } else { return StrIndex :: new (0 , 0) ; } let mut up = true ; let mut last_index = StrIndex :: new (0 , 0) ; for (char_index , (byte_index , c)) in iter { if up { if c . is_lowercase () { up = false ; } else { return last_index ; } } else if c . is_uppercase () { up = true ; last_index . byte_index = byte_index ; last_index . char_index = char_index ; } else if ! c . is_lowercase () { return StrIndex :: new (char_index , byte_index) ; } } if up { last_index } else { StrIndex :: new (s . chars () . count () , s . len ()) } }
};
}
