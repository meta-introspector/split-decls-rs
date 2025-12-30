// Generated macro for impl_25 (impl)
macro_rules! Depcrate_errorsimpl_25 {
() => {
// Module: crate::errors
// Provides: {"impl_25"}
// Dependencies: {}
impl SourceDestError { pub fn build (source : io :: Error , kind : SourceDestErrorKind , from_path : impl Into < PathBuf > , to_path : impl Into < PathBuf > ,) -> io :: Error { io :: Error :: new (source . kind () , Self { kind , source , from_path : from_path . into () , to_path : to_path . into () , } ,) } }
};
}
