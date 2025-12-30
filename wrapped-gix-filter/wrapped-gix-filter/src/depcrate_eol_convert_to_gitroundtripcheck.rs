// Generated macro for RoundTripCheck (enum)
macro_rules! Depcrate_eol_convert_to_gitRoundTripCheck {
() => {
// Module: crate::eol::convert_to_git
// Provides: {"RoundTripCheck"}
// Dependencies: {}
# [doc = " The kind of round-trip check to perform when converting line endings to `git`, i.e. `CRLF` to `LF`."] # [derive (Debug , Copy , Clone)] pub enum RoundTripCheck < 'a > { # [doc = " Fail with an error if conversion isn't round-trip safe."] Fail { # [doc = " The repository-relative path of the file to check. Used in case of error."] rela_path : & 'a Path , } , # [doc = " Emit a warning using `gix_trace::warn!`, but don't fail."] # [doc = ""] # [doc = " Note that the parent application has to setup tracing to make these events visible, along with a parent `span!`."] Warn { # [doc = " The repository-relative path of the file to check. Used in case of error."] rela_path : & 'a Path , } , }
};
}
