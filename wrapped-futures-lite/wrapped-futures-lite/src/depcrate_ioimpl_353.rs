// Generated macro for impl_353 (impl)
macro_rules! Depcrate_ioimpl_353 {
() => {
// Module: crate::io
// Provides: {"impl_353"}
// Dependencies: {}
impl < W : AsyncWrite + Unpin + ? Sized > Future for WriteAllFuture < '_ , W > { type Output = Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { writer , buf } = & mut * self ; while ! buf . is_empty () { let n = ready ! (Pin :: new (& mut ** writer) . poll_write (cx , buf)) ? ; let (_ , rest) = mem :: take (buf) . split_at (n) ; * buf = rest ; if n == 0 { return Poll :: Ready (Err (ErrorKind :: WriteZero . into ())) ; } } Poll :: Ready (Ok (())) } }
};
}
