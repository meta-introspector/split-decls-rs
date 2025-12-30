// Generated macro for Location (struct)
macro_rules! Depcrate_frameLocation {
() => {
// Module: crate::frame
// Provides: {"Location"}
// Dependencies: {}
# [doc = " A source location."] pub struct Location < 'a > { # [doc = " The file name."] pub file : Option < & 'a str > , # [doc = " The line number."] pub line : Option < u32 > , # [doc = " The column number."] # [doc = ""] # [doc = " A value of `Some(0)` indicates the left edge."] pub column : Option < u32 > , }
};
}
