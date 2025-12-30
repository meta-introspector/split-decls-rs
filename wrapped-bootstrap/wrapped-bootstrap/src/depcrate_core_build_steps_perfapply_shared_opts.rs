// Generated macro for apply_shared_opts (function)
macro_rules! Depcrate_core_build_steps_perfapply_shared_opts {
() => {
// Module: crate::core::build_steps::perf
// Provides: {"apply_shared_opts"}
// Dependencies: {}
fn apply_shared_opts (cmd : & mut BootstrapCommand , opts : & SharedOpts) { if ! opts . include . is_empty () { cmd . arg ("--include") . arg (opts . include . join (",")) ; } if ! opts . exclude . is_empty () { cmd . arg ("--exclude") . arg (opts . exclude . join (",")) ; } if ! opts . profiles . is_empty () { cmd . arg ("--profiles") . arg (opts . profiles . iter () . map (| p | p . to_string ()) . collect :: < Vec < _ > > () . join (",")) ; } if ! opts . scenarios . is_empty () { cmd . arg ("--scenarios") . arg (opts . scenarios . iter () . map (| p | p . to_string ()) . collect :: < Vec < _ > > () . join (",")) ; } }
};
}
