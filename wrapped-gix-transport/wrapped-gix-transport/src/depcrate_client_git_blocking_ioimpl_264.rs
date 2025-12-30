// Generated macro for impl_264 (impl)
macro_rules! Depcrate_client_git_blocking_ioimpl_264 {
() => {
// Module: crate::client::git::blocking_io
// Provides: {"impl_264"}
// Dependencies: {}
impl < R , W > client :: TransportWithoutIO for Connection < R , W > where R : std :: io :: Read , W : std :: io :: Write , { fn to_url (& self) -> Cow < '_ , BStr > { self . state . custom_url . as_ref () . map_or_else (| | { let mut possibly_lossy_url = self . state . path . clone () ; possibly_lossy_url . insert_str (0 , "file://") ; Cow :: Owned (possibly_lossy_url) } , | url | Cow :: Borrowed (url . as_ref ()) ,) } fn connection_persists_across_multiple_requests (& self) -> bool { true } fn configure (& mut self , _config : & dyn Any) -> Result < () , Box < dyn Error + Send + Sync + 'static > > { Ok (()) } }
};
}
