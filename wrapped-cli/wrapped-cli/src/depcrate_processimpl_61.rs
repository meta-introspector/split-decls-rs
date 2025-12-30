// Generated macro for impl_61 (impl)
macro_rules! Depcrate_processimpl_61 {
() => {
// Module: crate::process
// Provides: {"impl_61"}
// Dependencies: {}
impl CommandError { # [doc = " Create an error from an I/O error."] pub (crate) fn io (ioerr : io :: Error) -> CommandError { CommandError { kind : CommandErrorKind :: Io (ioerr) } } # [doc = " Create an error from the contents of stderr (which may be empty)."] pub (crate) fn stderr (bytes : Vec < u8 >) -> CommandError { CommandError { kind : CommandErrorKind :: Stderr (bytes) } } # [doc = " Returns true if and only if this error has empty data from stderr."] pub (crate) fn is_empty (& self) -> bool { match self . kind { CommandErrorKind :: Stderr (ref bytes) => bytes . is_empty () , _ => false , } } }
};
}
