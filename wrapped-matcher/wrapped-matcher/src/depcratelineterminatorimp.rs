// Generated macro for LineTerminatorImp (enum)
macro_rules! DepcrateLineTerminatorImp {
() => {
// Module: crate
// Provides: {"LineTerminatorImp"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] enum LineTerminatorImp { # [doc = " Any single byte representing a line terminator."] Byte (u8) , # [doc = " A line terminator represented by `\\r\\n`."] # [doc = ""] # [doc = " When this option is used, consumers may generally treat a lone `\\n` as"] # [doc = " a line terminator in addition to `\\r\\n`."] CRLF , }
};
}
