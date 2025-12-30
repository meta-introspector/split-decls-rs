// Generated macro for impl_72 (impl)
macro_rules! Depcrate_errorimpl_72 {
() => {
// Module: crate::error
// Provides: {"impl_72"}
// Dependencies: {}
impl fmt :: Display for ErrorKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { ErrorKind :: InvalidOutput => write ! (f , "Spawning the cargo subommand failed.") , ErrorKind :: CommandFailed => write ! (f , "The cargo subcommand returned an error.") , ErrorKind :: InvalidCommand => write ! (f , "Parsing the cargo subcommand's output failed.") , } } }
};
}
