// Generated macro for LineSeparator (enum)
macro_rules! DepcrateLineSeparator {
() => {
// Module: crate
// Provides: {"LineSeparator"}
// Dependencies: {}
# [doc = " Newline style"] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub enum LineSeparator { # [doc = " System-dependent line separator"] # [doc = ""] # [doc = " On UNIX system, uses \"\\n\""] # [doc = " On Windows system, uses \"\\r\\n\""] SystemDefault , # [doc = " Uses \"\\n\" as new line separator"] CR , # [doc = " Uses \"\\r\\n\" as new line separator"] CRLF , }
};
}
