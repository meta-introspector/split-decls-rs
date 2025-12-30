// Generated macro for impl_828 (impl)
macro_rules! Depcrate_stream_stream_peekimpl_828 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"impl_828"}
// Dependencies: {}
impl < 'a , St > Future for PeekMut < 'a , St > where St : Stream , { type Output = Option < & 'a mut St :: Item > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let inner = self . project () . inner ; if let Some (peekable) = inner { ready ! (peekable . as_mut () . poll_peek_mut (cx)) ; inner . take () . unwrap () . poll_peek_mut (cx) } else { panic ! ("PeekMut polled after completion") } } }
};
}
