// Generated macro for Profiler (trait)
macro_rules! Depcrate_timingProfiler {
() => {
// Module: crate::timing
// Provides: {"Profiler"}
// Dependencies: {}
# [doc = " A profiler."] pub trait Profiler { # [doc = " Start a profiling pass."] # [doc = ""] # [doc = " Will return a token which when dropped indicates the end of the pass."] # [doc = ""] # [doc = " Multiple passes can be active at the same time, but they must be started and stopped in a"] # [doc = " LIFO fashion."] fn start_pass (& self , pass : Pass) -> Box < dyn Any > ; }
};
}
