// Generated macro for count_match_end (function)
macro_rules! Depcrate_str_utilscount_match_end {
() => {
// Module: crate::str_utils
// Provides: {"count_match_end"}
// Dependencies: {}
# [doc = " Returns the number of chars and bytes that match from the end"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use clippy_utils::str_utils::{count_match_end, StrCount};"] # [doc = " assert_eq!(count_match_end(\"hello_cat\", \"bye_cat\"), StrCount::new(4, 4));"] # [doc = " assert_eq!(count_match_end(\"if_item_thing\", \"enum_value\"), StrCount::new(0, 0));"] # [doc = " assert_eq!(count_match_end(\"Clippy\", \"Clippy\"), StrCount::new(6, 6));"] # [doc = " assert_eq!(count_match_end(\"MyT\\u{f6}ff\", \"YourT\\u{f6}ff\"), StrCount::new(4, 5));"] # [doc = " ```"] # [must_use] pub fn count_match_end (str1 : & str , str2 : & str) -> StrCount { let char_count = str1 . chars () . count () ; if char_count == 0 { return StrCount :: default () ; } let iter1 = (0 .. char_count) . rev () . zip (str1 . chars () . rev ()) ; let byte_count = str2 . len () ; let iter2 = str2 . char_indices () . rev () ; iter1 . zip (iter2) . take_while (| ((_ , c1) , (_ , c2)) | c1 == c2) . last () . map_or_else (StrCount :: default , | ((char_index , _) , (byte_index , _)) | { StrCount :: new (char_count - char_index , byte_count - byte_index) }) }
};
}
