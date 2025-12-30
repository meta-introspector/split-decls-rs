// Generated macro for Position (struct)
macro_rules! Depcrate_astPosition {
() => {
// Module: crate::ast
// Provides: {"Position"}
// Dependencies: {}
# [doc = " A single position in a regular expression."] # [doc = ""] # [doc = " A position encodes one half of a span, and include the byte offset, line"] # [doc = " number and column number."] # [derive (Clone , Copy , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct Position { # [doc = " The absolute offset of this position, starting at `0` from the"] # [doc = " beginning of the regular expression pattern string."] pub offset : usize , # [doc = " The line number, starting at `1`."] pub line : usize , # [doc = " The approximate column number, starting at `1`."] pub column : usize , }
};
}
