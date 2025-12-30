// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl error :: Error for Error { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match * self { CouldNotExecuteCommand (ref e) => Some (e) , CommandError { .. } => None , Utf8Error (ref e) => Some (e) , UnexpectedVersionFormat => None , SemVerError (ref e) => Some (e) , UnknownPreReleaseTag (_) => None , LlvmVersionError (ref e) => Some (e) , } } }
};
}
