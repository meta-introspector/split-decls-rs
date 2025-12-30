// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl Credential for UnsupportedCredential { fn perform (& self , _registry : & RegistryInfo < '_ > , _action : & Action < '_ > , _args : & [& str] ,) -> Result < CredentialResponse , Error > { Err (Error :: UrlNotSupported) } }
};
}
