// Generated macro for impl_73 (impl)
macro_rules! Depcrate_client_blocking_io_fileimpl_73 {
() => {
// Module: crate::client::blocking_io::file
// Provides: {"impl_73"}
// Dependencies: {}
impl client :: TransportWithoutIO for SpawnProcessOnDemand { fn set_identity (& mut self , identity : gix_sec :: identity :: Account) -> Result < () , client :: Error > { if self . url . scheme == gix_url :: Scheme :: Ssh { self . url . set_user ((! identity . username . is_empty ()) . then_some (identity . username)) ; Ok (()) } else { Err (client :: Error :: AuthenticationUnsupported) } } fn to_url (& self) -> Cow < '_ , BStr > { Cow :: Owned (self . url . to_bstring ()) } fn connection_persists_across_multiple_requests (& self) -> bool { true } fn configure (& mut self , _config : & dyn Any) -> Result < () , Box < dyn Error + Send + Sync + 'static > > { Ok (()) } }
};
}
