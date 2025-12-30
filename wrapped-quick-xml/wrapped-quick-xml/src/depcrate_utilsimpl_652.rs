// Generated macro for impl_652 (impl)
macro_rules! Depcrate_utilsimpl_652 {
() => {
// Module: crate::utils
// Provides: {"impl_652"}
// Dependencies: {}
# [cfg (feature = "async-tokio")] impl < 'a > tokio :: io :: AsyncRead for Fountain < 'a > { fn poll_read (self : Pin < & mut Self > , _cx : & mut Context < '_ > , buf : & mut tokio :: io :: ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { let available = & self . chunk [self . consumed ..] ; let len = buf . remaining () . min (available . len ()) ; let (portion , _) = available . split_at (len) ; buf . put_slice (portion) ; Poll :: Ready (Ok (())) } }
};
}
