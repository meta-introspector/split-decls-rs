// Generated macro for run (function)
macro_rules! Depcrate_executorrun {
() => {
// Module: crate::executor
// Provides: {"run"}
// Dependencies: {}
pub (crate) fn run () { without_interrupts (| | { for _ in 0 .. 3 { if ! core_local :: ex () . try_tick () { break ; } } }) ; }
};
}
