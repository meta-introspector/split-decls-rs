// Generated macro for impl_20 (impl)
macro_rules! Depcrate_errorsimpl_20 {
() => {
// Module: crate::errors
// Provides: {"impl_20"}
// Dependencies: {}
impl Error { pub fn build (source : io :: Error , kind : ErrorKind , path : impl Into < PathBuf >) -> io :: Error { io :: Error :: new (source . kind () , Self { kind , source , path : path . into () , } ,) } }
};
}
