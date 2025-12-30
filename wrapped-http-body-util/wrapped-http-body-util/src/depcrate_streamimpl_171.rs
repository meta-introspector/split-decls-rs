// Generated macro for impl_171 (impl)
macro_rules! Depcrate_streamimpl_171 {
() => {
// Module: crate::stream
// Provides: {"impl_171"}
// Dependencies: {}
impl < B > Stream for BodyDataStream < B > where B : Body , { type Item = Result < B :: Data , B :: Error > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { loop { return match ready ! (self . as_mut () . project () . body . poll_frame (cx)) { Some (Ok (frame)) => match frame . into_data () { Ok (bytes) => Poll :: Ready (Some (Ok (bytes))) , Err (_) => continue , } , Some (Err (err)) => Poll :: Ready (Some (Err (err))) , None => Poll :: Ready (None) , } ; } } }
};
}
