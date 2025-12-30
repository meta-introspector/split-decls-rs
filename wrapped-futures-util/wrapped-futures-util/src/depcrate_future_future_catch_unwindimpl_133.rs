// Generated macro for impl_133 (impl)
macro_rules! Depcrate_future_future_catch_unwindimpl_133 {
() => {
// Module: crate::future::future::catch_unwind
// Provides: {"impl_133"}
// Dependencies: {}
impl < Fut > Future for CatchUnwind < Fut > where Fut : Future + UnwindSafe , { type Output = Result < Fut :: Output , Box < dyn Any + Send > > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let f = self . project () . future ; catch_unwind (AssertUnwindSafe (| | f . poll (cx))) ? . map (Ok) } }
};
}
