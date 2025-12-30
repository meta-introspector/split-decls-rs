// Generated macro for impl_44 (impl)
macro_rules! Depcrate_deriveimpl_44 {
() => {
// Module: crate::derive
// Provides: {"impl_44"}
// Dependencies: {}
impl FromArgMatches for Infallible { fn from_arg_matches (_matches : & ArgMatches) -> Result < Self , Error > { Err (Error :: raw (crate :: error :: ErrorKind :: MissingSubcommand , "a subcommand is required but one was not provided" ,)) } fn update_from_arg_matches (& mut self , _matches : & ArgMatches) -> Result < () , Error > { unreachable ! ("there will never be an instance of Infallible and thus &mut self can never be called") ; } }
};
}
