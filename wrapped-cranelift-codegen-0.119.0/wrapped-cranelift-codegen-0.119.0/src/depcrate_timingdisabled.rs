// Generated macro for disabled (module)
macro_rules! Depcrate_timingdisabled {
() => {
// Module: crate::timing
// Provides: {"disabled"}
// Dependencies: {}
# [cfg (not (feature = "timing"))] mod disabled { use super :: { DefaultProfiler , Pass , Profiler } ; use alloc :: boxed :: Box ; use core :: any :: Any ; impl Profiler for DefaultProfiler { fn start_pass (& self , _pass : Pass) -> Box < dyn Any > { Box :: new (()) } } pub (crate) fn start_pass (_pass : Pass) -> Box < dyn Any > { Box :: new (()) } }
};
}
