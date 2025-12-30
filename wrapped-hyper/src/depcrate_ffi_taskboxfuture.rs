// Generated macro for BoxFuture (type)
macro_rules! Depcrate_ffi_taskBoxFuture {
() => {
// Module: crate::ffi::task
// Provides: {"BoxFuture"}
// Dependencies: {}
type BoxFuture < T > = Pin < Box < dyn Future < Output = T > + Send > > ;
};
}
