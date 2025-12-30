// Generated macro for SourceDestError (struct)
macro_rules! Depcrate_errorsSourceDestError {
() => {
// Module: crate::errors
// Provides: {"SourceDestError"}
// Dependencies: {}
# [doc = " Error type used by functions like `fs::copy` that holds two paths."] # [derive (Debug)] pub (crate) struct SourceDestError { kind : SourceDestErrorKind , source : io :: Error , from_path : PathBuf , to_path : PathBuf , }
};
}
