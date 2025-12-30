// Generated macro for impl_149 (impl)
macro_rules! Depcrate_client_blocking_io_httpimpl_149 {
() => {
// Module: crate::client::blocking_io::http
// Provides: {"impl_149"}
// Dependencies: {}
impl < H : Http > client :: TransportWithoutIO for Transport < H > { fn set_identity (& mut self , identity : gix_sec :: identity :: Account) -> Result < () , client :: Error > { self . identity = Some (identity) ; Ok (()) } fn to_url (& self) -> Cow < '_ , BStr > { Cow :: Borrowed (self . url . as_str () . into ()) } fn connection_persists_across_multiple_requests (& self) -> bool { false } fn configure (& mut self , config : & dyn Any) -> Result < () , Box < dyn std :: error :: Error + Send + Sync + 'static > > { self . http . configure (config) } }
};
}
