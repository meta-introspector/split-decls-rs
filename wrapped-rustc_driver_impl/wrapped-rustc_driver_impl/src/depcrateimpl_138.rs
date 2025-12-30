// Generated macro for impl_138 (impl)
macro_rules! Depcrateimpl_138 {
() => {
// Module: crate
// Provides: {"impl_138"}
// Dependencies: {}
impl Callbacks for TimePassesCallbacks { # [allow (rustc :: bad_opt_access)] fn config (& mut self , config : & mut interface :: Config) { self . time_passes = (config . opts . prints . is_empty () && config . opts . unstable_opts . time_passes) . then (| | config . opts . unstable_opts . time_passes_format) ; config . opts . trimmed_def_paths = true ; } }
};
}
