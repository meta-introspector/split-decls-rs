// Generated macro for LineTerminator (struct)
macro_rules! DepcrateLineTerminator {
() => {
// Module: crate
// Provides: {"LineTerminator"}
// Dependencies: {}
# [doc = " A line terminator."] # [doc = ""] # [doc = " A line terminator represents the end of a line. Generally, every line is"] # [doc = " either \"terminated\" by the end of a stream or a specific byte (or sequence"] # [doc = " of bytes)."] # [doc = ""] # [doc = " Generally, a line terminator is a single byte, specifically, `\\n`, on"] # [doc = " Unix-like systems. On Windows, a line terminator is `\\r\\n` (referred to"] # [doc = " as `CRLF` for `Carriage Return; Line Feed`)."] # [doc = ""] # [doc = " The default line terminator is `\\n` on all platforms."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub struct LineTerminator (LineTerminatorImp) ;
};
}
