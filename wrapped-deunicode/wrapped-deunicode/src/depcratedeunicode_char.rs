// Generated macro for deunicode_char (function)
macro_rules! Depcratedeunicode_char {
() => {
// Module: crate
// Provides: {"deunicode_char"}
// Dependencies: {}
# [doc = " This function takes a single Unicode character and returns an ASCII"] # [doc = " transliteration."] # [doc = ""] # [doc = " The warnings and guarantees of [`deunicode()`] apply to this function as well."] # [doc = ""] # [doc = " Examples"] # [doc = " --------"] # [doc = " ```rust"] # [doc = " # use deunicode::deunicode_char;"] # [doc = " assert_eq!(deunicode_char('Æ'), Some(\"AE\"));"] # [doc = " assert_eq!(deunicode_char('北'), Some(\"Bei \"));"] # [doc = " ```"] # [inline] # [must_use] pub fn deunicode_char (ch : char) -> Option < & 'static str > { if let Some (p) = POINTERS . get (ch as usize) { if p . len <= 2 { let chars = p . chr . get (.. p . len as usize) ? ; debug_assert ! (core :: str :: from_utf8 (chars) . is_ok ()) ; unsafe { Some (core :: str :: from_utf8_unchecked (chars)) } } else { let map_pos = (u16 :: from (p . chr [0]) | u16 :: from (p . chr [1]) << 8) as usize ; MAPPING . get (map_pos .. map_pos + p . len as usize) } } else { None } }
};
}
