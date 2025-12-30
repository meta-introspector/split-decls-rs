// Generated macro for impl_14 (impl)
macro_rules! Depcrate_errorimpl_14 {
() => {
// Module: crate::error
// Provides: {"impl_14"}
// Dependencies: {}
impl ExitCode for CornError { fn get_exit_code (& self) -> i32 { match self { CornError :: Io (_) => 3 , CornError :: ParserError (_) => 1 , CornError :: InputResolveError (_) => 2 , CornError :: InvalidPathError (_) => 6 , CornError :: InvalidSpreadError (_) => 7 , CornError :: InvalidInterpolationError (_) => 8 , CornError :: DeserializationError (_) => 5 , } } }
};
}
