// Generated macro for split2 (function)
macro_rules! Depcrate_mimesplit2 {
() => {
// Module: crate::mime
// Provides: {"split2"}
// Dependencies: {}
fn split2 (s : & str , separator : char) -> (& str , Option < & str >) { let mut iter = s . splitn (2 , separator) ; let first = iter . next () . unwrap () ; (first , iter . next ()) }
};
}
