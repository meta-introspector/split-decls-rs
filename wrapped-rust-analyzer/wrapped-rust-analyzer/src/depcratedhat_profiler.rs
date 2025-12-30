// Generated macro for DHAT_PROFILER (static)
macro_rules! DepcrateDHAT_PROFILER {
() => {
// Module: crate
// Provides: {"DHAT_PROFILER"}
// Dependencies: {}
# [cfg (feature = "dhat")] static DHAT_PROFILER : std :: sync :: Mutex < Option < dhat :: Profiler > > = std :: sync :: Mutex :: new (None) ;
};
}
