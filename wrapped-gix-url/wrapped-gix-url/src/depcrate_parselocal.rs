// Generated macro for local (function)
macro_rules! Depcrate_parselocal {
() => {
// Module: crate::parse
// Provides: {"local"}
// Dependencies: {}
pub (crate) fn local (input : & BStr) -> Result < crate :: Url , Error > { if input . is_empty () { return Err (Error :: MissingRepositoryPath { url : input . to_owned () , kind : UrlKind :: Local , }) ; } Ok (crate :: Url { serialize_alternative_form : true , scheme : Scheme :: File , password : None , user : None , host : None , port : None , path : input . to_owned () , }) }
};
}
