// Generated macro for impl_75 (impl)
macro_rules! Depcrate_astimpl_75 {
() => {
// Module: crate::ast
// Provides: {"impl_75"}
// Dependencies: {}
impl Position { # [doc = " Create a new position with the given information."] # [doc = ""] # [doc = " `offset` is the absolute offset of the position, starting at `0` from"] # [doc = " the beginning of the regular expression pattern string."] # [doc = ""] # [doc = " `line` is the line number, starting at `1`."] # [doc = ""] # [doc = " `column` is the approximate column number, starting at `1`."] pub fn new (offset : usize , line : usize , column : usize) -> Position { Position { offset , line , column } } }
};
}
