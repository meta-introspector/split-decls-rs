// Generated macro for Credential (trait)
macro_rules! DepcrateCredential {
() => {
// Module: crate
// Provides: {"Credential"}
// Dependencies: {}
pub trait Credential { # [doc = " Retrieves a token for the given registry."] fn perform (& self , registry : & RegistryInfo < '_ > , action : & Action < '_ > , args : & [& str] ,) -> Result < CredentialResponse , Error > ; }
};
}
