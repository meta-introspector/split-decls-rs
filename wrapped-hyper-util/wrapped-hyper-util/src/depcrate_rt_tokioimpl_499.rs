// Generated macro for impl_499 (impl)
macro_rules! Depcrate_rt_tokioimpl_499 {
() => {
// Module: crate::rt::tokio
// Provides: {"impl_499"}
// Dependencies: {}
impl < T > tokio :: io :: AsyncRead for TokioIo < T > where T : hyper :: rt :: Read , { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , tbuf : & mut tokio :: io :: ReadBuf < '_ > ,) -> Poll < Result < () , std :: io :: Error > > { let filled = tbuf . filled () . len () ; let sub_filled = unsafe { let mut buf = hyper :: rt :: ReadBuf :: uninit (tbuf . unfilled_mut ()) ; match hyper :: rt :: Read :: poll_read (self . project () . inner , cx , buf . unfilled ()) { Poll :: Ready (Ok (())) => buf . filled () . len () , other => return other , } } ; let n_filled = filled + sub_filled ; let n_init = sub_filled ; unsafe { tbuf . assume_init (n_init) ; tbuf . set_filled (n_filled) ; } Poll :: Ready (Ok (())) } }
};
}
