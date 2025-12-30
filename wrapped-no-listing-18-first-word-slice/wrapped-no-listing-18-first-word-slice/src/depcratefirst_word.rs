// Generated macro for first_word (function)
macro_rules! Depcratefirst_word {
() => {
// Module: crate
// Provides: {"first_word"}
// Dependencies: {}
fn first_word (s : & String) -> & str { let bytes = s . as_bytes () ; for (i , & item) in bytes . iter () . enumerate () { if item == b' ' { return & s [0 .. i] ; } } & s [..] }
};
}
