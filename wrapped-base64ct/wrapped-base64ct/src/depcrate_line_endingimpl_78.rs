// Generated macro for impl_78 (impl)
macro_rules! Depcrate_line_endingimpl_78 {
() => {
// Module: crate::line_ending
// Provides: {"impl_78"}
// Dependencies: {}
impl Default for LineEnding { # [cfg (windows)] fn default () -> LineEnding { LineEnding :: CRLF } # [cfg (not (windows))] fn default () -> LineEnding { LineEnding :: LF } }
};
}
