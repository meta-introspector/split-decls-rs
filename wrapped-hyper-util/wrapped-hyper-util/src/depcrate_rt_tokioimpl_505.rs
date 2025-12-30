// Generated macro for impl_505 (impl)
macro_rules! Depcrate_rt_tokioimpl_505 {
() => {
// Module: crate::rt::tokio
// Provides: {"impl_505"}
// Dependencies: {}
impl TokioSleep { fn reset (self : Pin < & mut Self > , deadline : Instant) { self . project () . inner . as_mut () . reset (deadline . into ()) ; } }
};
}
