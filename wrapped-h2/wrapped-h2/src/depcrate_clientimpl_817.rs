// Generated macro for impl_817 (impl)
macro_rules! Depcrate_clientimpl_817 {
() => {
// Module: crate::client
// Provides: {"impl_817"}
// Dependencies: {}
impl Peer { pub fn convert_send_message (id : StreamId , request : Request < () > , protocol : Option < Protocol > , end_of_stream : bool ,) -> Result < Headers , SendError > { use http :: request :: Parts ; let (Parts { method , uri , headers , version , .. } , _ ,) = request . into_parts () ; let is_connect = method == Method :: CONNECT ; let mut pseudo = Pseudo :: request (method , uri , protocol) ; if pseudo . scheme . is_none () { if pseudo . authority . is_none () { if version == Version :: HTTP_2 { return Err (UserError :: MissingUriSchemeAndAuthority . into ()) ; } else { pseudo . set_scheme (uri :: Scheme :: HTTP) ; } } else if ! is_connect { } } let mut frame = Headers :: new (id , pseudo , headers) ; if end_of_stream { frame . set_end_stream () } Ok (frame) } }
};
}
