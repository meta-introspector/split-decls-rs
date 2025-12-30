// Generated macro for BoxFuture (type)
macro_rules! Depcrate_rt_async_supportBoxFuture {
() => {
// Module: crate::rt::async_support
// Provides: {"BoxFuture"}
// Dependencies: {}
type BoxFuture < 'a > = Pin < Box < dyn Future < Output = () > + 'a > > ;
};
}
