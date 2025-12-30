// Generated macro for erode_from_back (function)
macro_rules! Depcrate_needless_continueerode_from_back {
() => {
// Module: crate::needless_continue
// Provides: {"erode_from_back"}
// Dependencies: {}
# [doc = " Eats at `s` from the end till a closing brace `}` is encountered, and then continues eating"] # [doc = " till a non-whitespace character is found.  e.g., the string. If no closing `}` is present, the"] # [doc = " string will be preserved."] # [doc = ""] # [doc = " ```no_run"] # [doc = " {"] # [doc = "     let x = 5;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " is transformed to"] # [doc = ""] # [doc = " ```text"] # [doc = "     {"] # [doc = "         let x = 5;"] # [doc = " ```"] # [must_use] fn erode_from_back (s : & str) -> String { let mut ret = s . to_string () ; while ret . pop () . is_some_and (| c | c != '}') { } while let Some (c) = ret . pop () { if ! c . is_whitespace () { ret . push (c) ; break ; } } if ret . is_empty () { s . to_string () } else { ret } }
};
}
