// Generated macro for impl_327 (impl)
macro_rules! Depcrate_proto_peerimpl_327 {
() => {
// Module: crate::proto::peer
// Provides: {"impl_327"}
// Dependencies: {}
impl Dyn { pub fn is_server (& self) -> bool { * self == Dyn :: Server } pub fn is_local_init (& self , id : StreamId) -> bool { assert ! (! id . is_zero ()) ; self . is_server () == id . is_server_initiated () } pub fn convert_poll_message (& self , pseudo : Pseudo , fields : HeaderMap , stream_id : StreamId ,) -> Result < PollMessage , Error > { if self . is_server () { crate :: server :: Peer :: convert_poll_message (pseudo , fields , stream_id) . map (PollMessage :: Server) } else { crate :: client :: Peer :: convert_poll_message (pseudo , fields , stream_id) . map (PollMessage :: Client) } } # [doc = " Returns true if the remote peer can initiate a stream with the given ID."] pub fn ensure_can_open (& self , id : StreamId , mode : Open) -> Result < () , Error > { if self . is_server () { if mode . is_push_promise () || ! id . is_client_initiated () { proto_err ! (conn : "cannot open stream {:?} - not client initiated" , id) ; return Err (Error :: library_go_away (Reason :: PROTOCOL_ERROR)) ; } Ok (()) } else { if ! mode . is_push_promise () || ! id . is_server_initiated () { proto_err ! (conn : "cannot open stream {:?} - not server initiated" , id) ; return Err (Error :: library_go_away (Reason :: PROTOCOL_ERROR)) ; } Ok (()) } } }
};
}
