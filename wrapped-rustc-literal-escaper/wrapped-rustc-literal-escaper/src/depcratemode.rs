// Generated macro for Mode (enum)
macro_rules! DepcrateMode {
() => {
// Module: crate
// Provides: {"Mode"}
// Dependencies: {}
# [doc = " Enum of the different kinds of literal"] # [derive (Debug , Clone , Copy , PartialEq)] pub enum Mode { # [doc = " `'a'`"] Char , # [doc = " `b'a'`"] Byte , # [doc = " `\"hello\"`"] Str , # [doc = " `r\"hello\"`"] RawStr , # [doc = " `b\"hello\"`"] ByteStr , # [doc = " `br\"hello\"`"] RawByteStr , # [doc = " `c\"hello\"`"] CStr , # [doc = " `cr\"hello\"`"] RawCStr , }
};
}
