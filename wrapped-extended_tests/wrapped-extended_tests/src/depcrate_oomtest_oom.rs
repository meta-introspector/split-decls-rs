// Generated macro for test_oom (function)
macro_rules! Depcrate_oomtest_oom {
() => {
// Module: crate::oom
// Provides: {"test_oom"}
// Dependencies: {}
fn test_oom < F : FnOnce () + Send + UnwindSafe > (f : F) -> Result < () , Box < dyn Any + Send > > { let result = catch_unwind (| | { OOM_TEST . store (true , Relaxed) ; let _guard = ExitGuard :: new (| | { OOM_TEST . store (false , Relaxed) ; }) ; f () ; }) ; IN_PANIC . store (false , Relaxed) ; result }
};
}
