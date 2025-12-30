// Generated macro for DebugLoc (struct)
macro_rules! Depcrate_debuginfoDebugLoc {
() => {
// Module: crate::debuginfo
// Provides: {"DebugLoc"}
// Dependencies: {}
# [doc = " A source code location used to generate debug information."] pub struct DebugLoc { # [doc = " Information about the original source file."] pub file : Arc < SourceFile > , # [doc = " The (1-based) line number."] pub line : u32 , # [doc = " The (1-based) column number."] pub col : u32 , }
};
}
