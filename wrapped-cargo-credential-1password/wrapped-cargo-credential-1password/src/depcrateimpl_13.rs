// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl Credential for OnePasswordCredential { fn perform (& self , registry : & RegistryInfo < '_ > , action : & Action < '_ > , args : & [& str] ,) -> Result < CredentialResponse , Error > { let op = OnePasswordKeychain :: new (args) ? ; match action { Action :: Get (_) => { let session = op . signin () ? ; if let Some (id) = op . search (& session , registry . index_url) ? { op . get_token (& session , & id) . map (| token | CredentialResponse :: Get { token , cache : CacheControl :: Session , operation_independent : true , }) } else { Err (Error :: NotFound) } } Action :: Login (options) => { let session = op . signin () ? ; if let Some (id) = op . search (& session , registry . index_url) ? { eprintln ! ("note: token already exists for `{}`" , registry . index_url) ; let token = cargo_credential :: read_token (options , registry) ? ; op . modify (& session , & id , token . as_deref () , None) ? ; } else { let token = cargo_credential :: read_token (options , registry) ? ; op . create (& session , registry . index_url , token . as_deref () , None) ? ; } Ok (CredentialResponse :: Login) } Action :: Logout => { let session = op . signin () ? ; if let Some (id) = op . search (& session , registry . index_url) ? { op . delete (& session , & id) ? ; Ok (CredentialResponse :: Logout) } else { Err (Error :: NotFound) } } _ => Err (Error :: OperationNotSupported) , } } }
};
}
