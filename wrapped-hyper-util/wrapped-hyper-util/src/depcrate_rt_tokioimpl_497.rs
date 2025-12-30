// Generated macro for impl_497 (impl)
macro_rules! Depcrate_rt_tokioimpl_497 {
() => {
// Module: crate::rt::tokio
// Provides: {"impl_497"}
// Dependencies: {}
impl < T > hyper :: rt :: Read for TokioIo < T > where T : tokio :: io :: AsyncRead , { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , mut buf : hyper :: rt :: ReadBufCursor < '_ > ,) -> Poll < Result < () , std :: io :: Error > > { let n = unsafe { let mut tbuf = tokio :: io :: ReadBuf :: uninit (buf . as_mut ()) ; match tokio :: io :: AsyncRead :: poll_read (self . project () . inner , cx , & mut tbuf) { Poll :: Ready (Ok (())) => tbuf . filled () . len () , other => return other , } } ; unsafe { buf . advance (n) ; } Poll :: Ready (Ok (())) } }
};
}
