// Generated macro for delimiter_of_raw_string (function)
macro_rules! Depcrate_parsedelimiter_of_raw_string {
() => {
// Module: crate::parse
// Provides: {"delimiter_of_raw_string"}
// Dependencies: {}
fn delimiter_of_raw_string (input : Cursor) -> PResult < & str > { for (i , byte) in input . bytes () . enumerate () { match byte { b'"' => { if i > 255 { return Err (Reject) ; } return Ok ((input . advance (i + 1) , & input . rest [.. i])) ; } b'#' => { } _ => break , } } Err (Reject) }
};
}
