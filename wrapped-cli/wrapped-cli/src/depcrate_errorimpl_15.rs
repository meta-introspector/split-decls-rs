// Generated macro for impl_15 (impl)
macro_rules! Depcrate_errorimpl_15 {
() => {
// Module: crate::error
// Provides: {"impl_15"}
// Dependencies: {}
impl ExitCode for Error { fn get_exit_code (& self) -> i32 { match self { Error :: Corn (err) => err . get_exit_code () , Error :: ReadingFile (_) => 3 , Error :: Serializing (_) => 4 , } } }
};
}
