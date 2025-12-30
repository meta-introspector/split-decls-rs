// Generated macro for count_match_start (function)
macro_rules! Depcrate_str_utilscount_match_start {
() => {
// Module: crate::str_utils
// Provides: {"count_match_start"}
// Dependencies: {}
# [doc = " Returns the number of chars that match from the start"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use clippy_utils::str_utils::{count_match_start, StrCount};"] # [doc = " assert_eq!(count_match_start(\"hello_mouse\", \"hello_penguin\"), StrCount::new(6, 6));"] # [doc = " assert_eq!(count_match_start(\"hello_clippy\", \"bye_bugs\"), StrCount::new(0, 0));"] # [doc = " assert_eq!(count_match_start(\"hello_world\", \"hello_world\"), StrCount::new(11, 11));"] # [doc = " assert_eq!(count_match_start(\"T\\u{f6}ffT\\u{f6}ff\", \"T\\u{f6}ff\"), StrCount::new(4, 5));"] # [doc = " ```"] # [must_use] pub fn count_match_start (str1 : & str , str2 : & str) -> StrCount { let char_count = str1 . chars () . count () ; let iter1 = (0 ..= char_count) . zip (str1 . chars ()) ; let iter2 = str2 . char_indices () ; iter1 . zip (iter2) . take_while (| ((_ , c1) , (_ , c2)) | c1 == c2) . last () . map_or_else (StrCount :: default , | ((char_index , _) , (byte_index , character)) | { StrCount :: new (char_index + 1 , byte_index + character . len_utf8 ()) }) }
};
}
