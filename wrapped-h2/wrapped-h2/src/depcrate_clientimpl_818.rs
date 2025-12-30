// Generated macro for impl_818 (impl)
macro_rules! Depcrate_clientimpl_818 {
() => {
// Module: crate::client
// Provides: {"impl_818"}
// Dependencies: {}
impl proto :: Peer for Peer { type Poll = Response < () > ; const NAME : & 'static str = "Client" ; fn r#dyn () -> proto :: DynPeer { proto :: DynPeer :: Client } fn convert_poll_message (pseudo : Pseudo , fields : HeaderMap , stream_id : StreamId ,) -> Result < Self :: Poll , Error > { let mut b = Response :: builder () ; b = b . version (Version :: HTTP_2) ; if let Some (status) = pseudo . status { b = b . status (status) ; } let mut response = match b . body (()) { Ok (response) => response , Err (_) => { return Err (Error :: library_reset (stream_id , Reason :: PROTOCOL_ERROR)) ; } } ; * response . headers_mut () = fields ; Ok (response) } }
};
}
