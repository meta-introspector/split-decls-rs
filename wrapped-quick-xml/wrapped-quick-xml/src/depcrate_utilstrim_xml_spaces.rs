// Generated macro for trim_xml_spaces (function)
macro_rules! Depcrate_utilstrim_xml_spaces {
() => {
// Module: crate::utils
// Provides: {"trim_xml_spaces"}
// Dependencies: {}
# [doc = " Returns a string slice with XML whitespace characters removed from both sides."] # [doc = ""] # [doc = " 'Whitespace' refers to the definition used by [`is_whitespace`]."] # [inline] pub fn trim_xml_spaces (text : & str) -> & str { let bytes = trim_xml_end (trim_xml_start (text . as_bytes ())) ; match core :: str :: from_utf8 (bytes) { Ok (s) => s , _ => unreachable ! () , } }
};
}
