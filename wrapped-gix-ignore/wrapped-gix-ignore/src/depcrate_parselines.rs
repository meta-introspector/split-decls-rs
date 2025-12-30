// Generated macro for Lines (struct)
macro_rules! Depcrate_parseLines {
() => {
// Module: crate::parse
// Provides: {"Lines"}
// Dependencies: {}
# [doc = " An iterator over line-wise ignore patterns parsed from a buffer."] pub struct Lines < 'a > { lines : bstr :: Lines < 'a > , line_no : usize , # [doc = " Only if `true` we will be able to parse precious files."] support_precious : bool , }
};
}
