// Generated macro for impl_460 (impl)
macro_rules! Depcrate_core_build_steps_perfimpl_460 {
() => {
// Module: crate::core::build_steps::perf
// Provides: {"impl_460"}
// Dependencies: {}
impl PerfCommand { fn shared_opts (& self) -> Option < & SharedOpts > { match self { PerfCommand :: Eprintln { opts , .. } | PerfCommand :: Samply { opts , .. } | PerfCommand :: Cachegrind { opts , .. } | PerfCommand :: Benchmark { opts , .. } => Some (opts) , PerfCommand :: Compare { .. } => None , } } }
};
}
