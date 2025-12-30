// Generated macro for impl_78 (impl)
macro_rules! Depcrate_runnableimpl_78 {
() => {
// Module: crate::runnable
// Provides: {"impl_78"}
// Dependencies: {}
impl < M , F > Schedule < M > for F where F : Fn (Runnable < M >) , { fn schedule (& self , runnable : Runnable < M > , _ : ScheduleInfo) { self (runnable) } }
};
}
