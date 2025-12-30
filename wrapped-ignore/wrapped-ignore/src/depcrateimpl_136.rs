// Generated macro for impl_136 (impl)
macro_rules! Depcrateimpl_136 {
() => {
// Module: crate
// Provides: {"impl_136"}
// Dependencies: {}
impl Clone for Error { fn clone (& self) -> Error { match * self { Error :: Partial (ref errs) => Error :: Partial (errs . clone ()) , Error :: WithLineNumber { line , ref err } => { Error :: WithLineNumber { line , err : err . clone () } } Error :: WithPath { ref path , ref err } => { Error :: WithPath { path : path . clone () , err : err . clone () } } Error :: WithDepth { depth , ref err } => { Error :: WithDepth { depth , err : err . clone () } } Error :: Loop { ref ancestor , ref child } => Error :: Loop { ancestor : ancestor . clone () , child : child . clone () , } , Error :: Io (ref err) => match err . raw_os_error () { Some (e) => Error :: Io (std :: io :: Error :: from_raw_os_error (e)) , None => { Error :: Io (std :: io :: Error :: new (err . kind () , err . to_string ())) } } , Error :: Glob { ref glob , ref err } => { Error :: Glob { glob : glob . clone () , err : err . clone () } } Error :: UnrecognizedFileType (ref err) => { Error :: UnrecognizedFileType (err . clone ()) } Error :: InvalidDefinition => Error :: InvalidDefinition , } } }
};
}
