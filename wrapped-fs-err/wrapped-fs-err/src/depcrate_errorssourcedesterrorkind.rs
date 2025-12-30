// Generated macro for SourceDestErrorKind (enum)
macro_rules! Depcrate_errorsSourceDestErrorKind {
() => {
// Module: crate::errors
// Provides: {"SourceDestErrorKind"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] pub (crate) enum SourceDestErrorKind { Copy , HardLink , Rename , SoftLink , # [cfg (unix)] Symlink , # [cfg (windows)] SymlinkDir , # [cfg (windows)] SymlinkFile , }
};
}
