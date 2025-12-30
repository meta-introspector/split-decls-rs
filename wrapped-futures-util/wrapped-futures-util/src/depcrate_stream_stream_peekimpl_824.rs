// Generated macro for impl_824 (impl)
macro_rules! Depcrate_stream_stream_peekimpl_824 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"impl_824"}
// Dependencies: {}
impl < 'a , St > Future for Peek < 'a , St > where St : Stream , { type Output = Option < & 'a St :: Item > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let inner = self . project () . inner ; if let Some (peekable) = inner { ready ! (peekable . as_mut () . poll_peek (cx)) ; inner . take () . unwrap () . poll_peek (cx) } else { panic ! ("Peek polled after completion") } } }
};
}
