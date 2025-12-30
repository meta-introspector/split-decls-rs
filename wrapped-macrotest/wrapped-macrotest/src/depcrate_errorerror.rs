// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum Error { Cargo (std :: io :: Error) , CargoExpandExecution (String) , CargoFail , CargoMetadata (serde_json :: error :: Error) , Io (std :: io :: Error) , TomlSer (toml :: ser :: Error) , TomlDe (toml :: de :: Error) , Glob (glob :: GlobError) , GlobPattern (glob :: PatternError) , ManifestDir , PkgName , UnrecognizedEnv (std :: ffi :: OsString) , }
};
}
