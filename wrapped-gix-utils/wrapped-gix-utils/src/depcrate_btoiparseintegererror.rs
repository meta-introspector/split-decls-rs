// Generated macro for ParseIntegerError (struct)
macro_rules! Depcrate_btoiParseIntegerError {
() => {
// Module: crate::btoi
// Provides: {"ParseIntegerError"}
// Dependencies: {}
# [doc = " An error that can occur when parsing an integer."] # [doc = ""] # [doc = " * No digits"] # [doc = " * Invalid digit"] # [doc = " * Overflow"] # [doc = " * Underflow"] # [derive (Debug , Clone , PartialEq , Eq)] pub struct ParseIntegerError { kind : ErrorKind , }
};
}
