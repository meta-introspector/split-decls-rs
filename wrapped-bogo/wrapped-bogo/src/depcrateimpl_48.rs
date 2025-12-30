// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
impl MultipleClientCredentialResolver { fn add (& mut self , key : Credentials , meta : & Credential) { self . additional . push (ClientCert :: new (key , meta)) ; } fn set_default (& mut self , key : Credentials , meta : & Credential) { self . default = Some (ClientCert :: new (key , meta)) ; } }
};
}
