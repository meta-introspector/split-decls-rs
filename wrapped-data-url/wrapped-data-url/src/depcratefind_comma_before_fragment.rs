// Generated macro for find_comma_before_fragment (function)
macro_rules! Depcratefind_comma_before_fragment {
() => {
// Module: crate
// Provides: {"find_comma_before_fragment"}
// Dependencies: {}
fn find_comma_before_fragment (after_colon : & str) -> Option < (& str , & str) > { for (i , byte) in after_colon . bytes () . enumerate () { if byte == b',' { return Some ((& after_colon [.. i] , & after_colon [i + 1 ..])) ; } if byte == b'#' { break ; } } None }
};
}
