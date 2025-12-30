// Generated macro for impl_45 (impl)
macro_rules! Depcrate_futureimpl_45 {
() => {
// Module: crate::future
// Provides: {"impl_45"}
// Dependencies: {}
# [cfg (feature = "std")] impl < F : Future + UnwindSafe > Future for CatchUnwind < F > { type Output = Result < F :: Output , Box < dyn Any + Send > > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; catch_unwind (AssertUnwindSafe (| | this . inner . poll (cx))) ? . map (Ok) } }
};
}
