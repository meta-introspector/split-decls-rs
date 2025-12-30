// Generated macro for LineCol (struct)
macro_rules! DepcrateLineCol {
() => {
// Module: crate
// Provides: {"LineCol"}
// Dependencies: {}
# [doc = " `(line, column)` information in the native, UTF-8 encoding."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct LineCol { # [doc = " Zero-based."] pub line : u32 , # [doc = " Zero-based UTF-8 offset."] pub col : u32 , }
};
}
