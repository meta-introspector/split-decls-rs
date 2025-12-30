// Generated macro for impl_46 (impl)
macro_rules! Depcrate_streamimpl_46 {
() => {
// Module: crate::stream
// Provides: {"impl_46"}
// Dependencies: {}
impl < T : Write + Read + Connection + Unpin > Connection for MaybeHttpsStream < T > { fn connected (& self) -> Connected { match self { MaybeHttpsStream :: Http (s) => s . connected () , MaybeHttpsStream :: Https (s) => { let c = s . inner () . get_ref () . get_ref () . get_ref () . inner () . connected () ; # [cfg (feature = "alpn")] { if negotiated_h2 (s . inner () . get_ref ()) { return c . negotiated_h2 () ; } } c } } } }
};
}
