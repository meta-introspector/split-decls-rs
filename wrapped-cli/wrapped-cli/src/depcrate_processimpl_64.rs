// Generated macro for impl_64 (impl)
macro_rules! Depcrate_processimpl_64 {
() => {
// Module: crate::process
// Provides: {"impl_64"}
// Dependencies: {}
impl From < io :: Error > for CommandError { fn from (ioerr : io :: Error) -> CommandError { CommandError { kind : CommandErrorKind :: Io (ioerr) } } }
};
}
