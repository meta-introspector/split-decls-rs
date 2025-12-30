// Generated macro for impl_59 (impl)
macro_rules! Depcrate_errorimpl_59 {
() => {
// Module: crate::error
// Provides: {"impl_59"}
// Dependencies: {}
impl std :: fmt :: Display for Error { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { use self :: Error :: * ; match self { Cargo (e) => write ! (f , "{}" , e) , CargoExpandExecution (e) => write ! (f , "Failed to execute cargo command: {}" , e) , CargoFail => write ! (f , "cargo reported an error") , CargoMetadata (e) => write ! (f , "{}" , e) , Io (e) => write ! (f , "{}" , e) , TomlSer (e) => write ! (f , "{}" , e) , TomlDe (e) => write ! (f , "{}" , e) , Glob (e) => write ! (f , "{}" , e) , GlobPattern (e) => write ! (f , "{}" , e) , ManifestDir => write ! (f , "could not find CARGO_MANIFEST_DIR env var") , PkgName => write ! (f , "could not find CARGO_PKG_NAME env var") , UnrecognizedEnv (e) => write ! (f , "unrecognized value of MACROTEST: \"{}\"" , e . to_string_lossy ()) , } } }
};
}
