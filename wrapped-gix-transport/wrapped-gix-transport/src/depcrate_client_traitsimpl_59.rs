// Generated macro for impl_59 (impl)
macro_rules! Depcrate_client_traitsimpl_59 {
() => {
// Module: crate::client::traits
// Provides: {"impl_59"}
// Dependencies: {}
impl < T : TransportWithoutIO + ? Sized > TransportWithoutIO for & mut T { fn set_identity (& mut self , identity : gix_sec :: identity :: Account) -> Result < () , Error > { self . deref_mut () . set_identity (identity) } fn to_url (& self) -> Cow < '_ , BStr > { self . deref () . to_url () } fn supported_protocol_versions (& self) -> & [Protocol] { self . deref () . supported_protocol_versions () } fn connection_persists_across_multiple_requests (& self) -> bool { self . deref () . connection_persists_across_multiple_requests () } fn configure (& mut self , config : & dyn Any) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { self . deref_mut () . configure (config) } }
};
}
