// Generated macro for Mode (enum)
macro_rules! Depcrate_eolMode {
() => {
// Module: crate::eol
// Provides: {"Mode"}
// Dependencies: {}
# [doc = " The kind of end of lines to set."] # [doc = ""] # [doc = " The default is implemented to be the native line ending for the current platform."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub enum Mode { # [doc = " Equivalent to `git` (`\\n`) line-endings."] Lf , # [doc = " Equivalent to `windows` (`\\r\\n`) line-endings."] CrLf , }
};
}
