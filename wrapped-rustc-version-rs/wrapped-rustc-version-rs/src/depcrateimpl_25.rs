// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { CouldNotExecuteCommand (ref e) => write ! (f , "could not execute command: {}" , e) , CommandError { ref stdout , ref stderr , } => write ! (f , "error from command -- stderr:\n\n{}\n\nstderr:\n\n{}" , stderr , stdout ,) , Utf8Error (_) => write ! (f , "invalid UTF-8 output from `rustc -vV`") , UnexpectedVersionFormat => write ! (f , "unexpected `rustc -vV` format") , SemVerError (ref e) => write ! (f , "error parsing version: {}" , e) , UnknownPreReleaseTag (ref i) => write ! (f , "unknown pre-release tag: {}" , i) , LlvmVersionError (ref e) => write ! (f , "error parsing LLVM's version: {}" , e) , } } }
};
}
