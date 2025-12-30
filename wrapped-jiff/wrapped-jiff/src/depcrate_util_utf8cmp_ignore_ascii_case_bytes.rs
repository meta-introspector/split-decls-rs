// Generated macro for cmp_ignore_ascii_case_bytes (function)
macro_rules! Depcrate_util_utf8cmp_ignore_ascii_case_bytes {
() => {
// Module: crate::util::utf8
// Provides: {"cmp_ignore_ascii_case_bytes"}
// Dependencies: {}
# [doc = " Like std's `eq_ignore_ascii_case`, but returns a full `Ordering` on"] # [doc = " `&[u8]`."] # [inline] pub (crate) fn cmp_ignore_ascii_case_bytes (s1 : & [u8] , s2 : & [u8]) -> Ordering { let mut i = 0 ; loop { let b1 = s1 . get (i) . copied () . map (| b | b . to_ascii_lowercase ()) ; let b2 = s2 . get (i) . copied () . map (| b | b . to_ascii_lowercase ()) ; match (b1 , b2) { (None , None) => return Ordering :: Equal , (Some (_) , None) => return Ordering :: Greater , (None , Some (_)) => return Ordering :: Less , (Some (b1) , Some (b2)) if b1 == b2 => i += 1 , (Some (b1) , Some (b2)) => return b1 . cmp (& b2) , } } }
};
}
