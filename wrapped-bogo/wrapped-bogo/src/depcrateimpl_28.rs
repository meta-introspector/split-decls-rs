// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl CredentialSet { fn last_mut (& mut self) -> & mut Credential { self . additional . last_mut () . unwrap_or (& mut self . default) } fn configured (& self) -> bool { self . default . configured () || self . additional . iter () . any (| cred | cred . configured ()) } }
};
}
