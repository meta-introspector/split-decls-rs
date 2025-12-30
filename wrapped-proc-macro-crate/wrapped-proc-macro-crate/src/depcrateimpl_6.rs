// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Error :: NotFound (path) => write ! (f , "Could not find `Cargo.toml` in manifest dir: `{}`." , path . display ()) , Error :: CargoManifestDirNotSet => f . write_str ("`CARGO_MANIFEST_DIR` env variable not set.") , Error :: CouldNotRead { path , .. } => write ! (f , "Could not read `{}`." , path . display ()) , Error :: InvalidToml { .. } => f . write_str ("Invalid toml file.") , Error :: CrateNotFound { crate_name , path } => write ! (f , "Could not find `{}` in `dependencies` or `dev-dependencies` in `{}`!" , crate_name , path . display () ,) , Error :: FailedGettingWorkspaceManifestPath => f . write_str ("Failed to get the path of the workspace manifest path.") , } } }
};
}
