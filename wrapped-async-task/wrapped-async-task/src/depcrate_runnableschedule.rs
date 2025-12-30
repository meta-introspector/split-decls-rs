// Generated macro for Schedule (trait)
macro_rules! Depcrate_runnableSchedule {
() => {
// Module: crate::runnable
// Provides: {"Schedule"}
// Dependencies: {}
# [doc = " The trait for scheduling functions."] pub trait Schedule < M = () > : sealed :: Sealed < M > { # [doc = " The actual scheduling procedure."] fn schedule (& self , runnable : Runnable < M > , info : ScheduleInfo) ; }
};
}
