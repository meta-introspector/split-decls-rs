// Generated macro for impl_312 (impl)
macro_rules! Depcrate_sleepimpl_312 {
() => {
// Module: crate::sleep
// Provides: {"impl_312"}
// Dependencies: {}
impl IdleState { fn wake_fully (& mut self) { self . rounds = 0 ; self . jobs_counter = JobsEventCounter :: DUMMY ; } fn wake_partly (& mut self) { self . rounds = ROUNDS_UNTIL_SLEEPY ; self . jobs_counter = JobsEventCounter :: DUMMY ; } }
};
}
