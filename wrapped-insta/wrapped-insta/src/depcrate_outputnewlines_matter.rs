// Generated macro for newlines_matter (function)
macro_rules! Depcrate_outputnewlines_matter {
() => {
// Module: crate::output
// Provides: {"newlines_matter"}
// Dependencies: {}
fn newlines_matter (left : & str , right : & str) -> bool { if trailing_newline (left) != trailing_newline (right) { return true ; } let (cr1 , crlf1 , lf1) = detect_newlines (left) ; let (cr2 , crlf2 , lf2) = detect_newlines (right) ; ! matches ! ((cr1 || cr2 , crlf1 || crlf2 , lf1 || lf2) , (false , false , false) | (true , false , false) | (false , true , false) | (false , false , true)) }
};
}
