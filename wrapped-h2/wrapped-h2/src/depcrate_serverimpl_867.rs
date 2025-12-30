// Generated macro for impl_867 (impl)
macro_rules! Depcrate_serverimpl_867 {
() => {
// Module: crate::server
// Provides: {"impl_867"}
// Dependencies: {}
impl Peer { pub fn convert_send_message (id : StreamId , response : Response < () > , end_of_stream : bool ,) -> frame :: Headers { use http :: response :: Parts ; let (Parts { status , headers , .. } , _ ,) = response . into_parts () ; let pseudo = Pseudo :: response (status) ; let mut frame = frame :: Headers :: new (id , pseudo , headers) ; if end_of_stream { frame . set_end_stream () } frame } pub fn convert_push_message (stream_id : StreamId , promised_id : StreamId , request : Request < () > ,) -> Result < frame :: PushPromise , UserError > { use http :: request :: Parts ; if let Err (e) = frame :: PushPromise :: validate_request (& request) { use PushPromiseHeaderError :: * ; match e { NotSafeAndCacheable => tracing :: debug ! (? promised_id , "convert_push_message: method {} is not safe and cacheable" , request . method () ,) , InvalidContentLength (e) => tracing :: debug ! (? promised_id , "convert_push_message; promised request has invalid content-length {:?}" , e ,) , } return Err (UserError :: MalformedHeaders) ; } let (Parts { method , uri , headers , .. } , _ ,) = request . into_parts () ; let pseudo = Pseudo :: request (method , uri , None) ; Ok (frame :: PushPromise :: new (stream_id , promised_id , pseudo , headers ,)) } }
};
}
