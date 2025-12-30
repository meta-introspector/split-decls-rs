// Generated macro for impl_65 (impl)
macro_rules! Depcrate_processimpl_65 {
() => {
// Module: crate::process
// Provides: {"impl_65"}
// Dependencies: {}
impl From < CommandError > for io :: Error { fn from (cmderr : CommandError) -> io :: Error { match cmderr . kind { CommandErrorKind :: Io (ioerr) => ioerr , CommandErrorKind :: Stderr (_) => { io :: Error :: new (io :: ErrorKind :: Other , cmderr) } } } }
};
}
