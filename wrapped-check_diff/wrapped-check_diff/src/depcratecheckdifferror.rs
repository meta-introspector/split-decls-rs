// Generated macro for CheckDiffError (enum)
macro_rules! DepcrateCheckDiffError {
() => {
// Module: crate
// Provides: {"CheckDiffError"}
// Dependencies: {}
# [derive (Debug)] pub enum CheckDiffError { # [doc = " Git related errors"] FailedGit (GitError) , # [doc = " Error for generic commands"] FailedCommand (& 'static str) , # [doc = " UTF8 related errors"] FailedUtf8 (Utf8Error) , # [doc = " Error for building rustfmt from source"] FailedSourceBuild (& 'static str) , # [doc = " Error when obtaining binary version"] FailedBinaryVersioning (PathBuf) , # [doc = " Error when obtaining cargo version"] FailedCargoVersion (& 'static str) , IO (std :: io :: Error) , }
};
}
