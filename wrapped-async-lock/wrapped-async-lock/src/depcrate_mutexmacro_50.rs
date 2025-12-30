// Generated macro for macro_50 (macro)
macro_rules! Depcrate_mutexmacro_50 {
() => {
// Module: crate::mutex
// Provides: {"macro_50"}
// Dependencies: {}
pin_project_lite :: pin_project ! { # [project = LockArcInnardsProj] enum LockArcInnards < T : ? Sized > { # [doc = " We have not tried to poll the fast path yet."] Unpolled { mutex : Option < Arc < Mutex < T >>> } , # [doc = " We are acquiring the mutex through the slow path."] AcquireSlow { # [pin] inner : AcquireSlow < Arc < Mutex < T >>, T > } , } }
};
}
