// Generated macro for err (function)
macro_rules! Depcrate_de_testserr {
() => {
// Module: crate::de::tests
// Provides: {"err"}
// Dependencies: {}
fn err < T > (kind : Error , (line_start , col_start) : (usize , usize) , (line_end , col_end) : (usize , usize) ,) -> SpannedResult < T > { Err (SpannedError { code : kind , span : Span { start : Position { line : line_start , col : col_start , } , end : Position { line : line_end , col : col_end , } , } , }) }
};
}
