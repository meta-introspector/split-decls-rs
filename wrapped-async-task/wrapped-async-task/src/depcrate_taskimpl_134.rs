// Generated macro for impl_134 (impl)
macro_rules! Depcrate_taskimpl_134 {
() => {
// Module: crate::task
// Provides: {"impl_134"}
// Dependencies: {}
impl < T , M > Future for FallibleTask < T , M > { type Output = Option < T > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { poll_task :: < T > (self . task . ptr . as_ptr () , cx) } }
};
}
