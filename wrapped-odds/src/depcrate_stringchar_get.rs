// Generated macro for char_get (function)
macro_rules! Depcrate_stringchar_get {
() => {
// Module: crate::string
// Provides: {"char_get"}
// Dependencies: {}
fn char_get (s : & str , i : usize) -> Option < char > { s [i ..] . chars () . next () }
};
}
