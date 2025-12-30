// Generated macro for impl_81 (impl)
macro_rules! Depcrate_runnableimpl_81 {
() => {
// Module: crate::runnable
// Provides: {"impl_81"}
// Dependencies: {}
impl < M , F > Schedule < M > for WithInfo < F > where F : Fn (Runnable < M > , ScheduleInfo) , { fn schedule (& self , runnable : Runnable < M > , info : ScheduleInfo) { (self . 0) (runnable , info) } }
};
}
