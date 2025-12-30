// Generated macro for impl_338 (impl)
macro_rules! Depcrate_future_poll_immediateimpl_338 {
() => {
// Module: crate::future::poll_immediate
// Provides: {"impl_338"}
// Dependencies: {}
impl < T , F > Future for PollImmediate < F > where F : Future < Output = T > , { type Output = Option < T > ; # [inline] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < T > > { let mut this = self . project () ; let inner = this . future . as_mut () . as_pin_mut () . expect ("PollImmediate polled after completion") ; match inner . poll (cx) { Poll :: Ready (t) => { this . future . set (None) ; Poll :: Ready (Some (t)) } Poll :: Pending => Poll :: Ready (None) , } } }
};
}
