// Generated macro for impl_49 (impl)
macro_rules! Depcrate_streamimpl_49 {
() => {
// Module: crate::stream
// Provides: {"impl_49"}
// Dependencies: {}
impl < T : rt :: Read + rt :: Write + Connection + Unpin > Connection for MaybeHttpsStream < T > { fn connected (& self) -> Connected { match self { Self :: Http (s) => s . connected () , Self :: Https (s) => { let (tcp , tls) = s . inner () . get_ref () ; if tls . alpn_protocol () == Some (b"h2") { tcp . inner () . connected () . negotiated_h2 () } else { tcp . inner () . connected () } } } } }
};
}
