// Generated macro for internal (module)
macro_rules! Depcrate_test_runner_scoped_panic_hookinternal {
() => {
// Module: crate::test_runner::scoped_panic_hook
// Provides: {"internal"}
// Dependencies: {}
# [cfg (not (feature = "handle-panics"))] mod internal { use core :: panic :: PanicInfo ; # [doc = " Simply executes `body` and returns its execution result."] # [doc = " Hook parameter is ignored"] pub fn with_hook < R > (_ : impl FnMut (& PanicInfo < '_ >) , body : impl FnOnce () -> R ,) -> R { body () } }
};
}
