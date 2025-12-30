// Generated macro for run_test (function)
macro_rules! Depcrate_oomrun_test {
() => {
// Module: crate::oom
// Provides: {"run_test"}
// Dependencies: {}
fn run_test < F : FnOnce (usize) > (f : F , repeat : usize) { PANIC_COUNT . store (0 , Relaxed) ; f (repeat) ; while INST_CNT . load (Relaxed) != 0 { let _ : Result < () , Box < dyn Any + Send > > = catch_unwind (| | { OOM_TEST . store (true , Relaxed) ; let _guard = ExitGuard :: new (| | { OOM_TEST . store (false , Relaxed) ; }) ; drop (Guard :: new ()) ; }) ; yield_now () ; } }
};
}
