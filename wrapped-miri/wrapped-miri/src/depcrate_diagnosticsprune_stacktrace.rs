// Generated macro for prune_stacktrace (function)
macro_rules! Depcrate_diagnosticsprune_stacktrace {
() => {
// Module: crate::diagnostics
// Provides: {"prune_stacktrace"}
// Dependencies: {}
# [doc = " Attempts to prune a stacktrace to omit the Rust runtime, and returns a bool indicating if any"] # [doc = " frames were pruned. If the stacktrace does not have any local frames, we conclude that it must"] # [doc = " be pointing to a problem in the Rust runtime itself, and do not prune it at all."] pub fn prune_stacktrace < 'tcx > (mut stacktrace : Vec < FrameInfo < 'tcx > > , machine : & MiriMachine < 'tcx > ,) -> (Vec < FrameInfo < 'tcx > > , bool) { match machine . backtrace_style { BacktraceStyle :: Off => { stacktrace . retain (| frame | ! frame . instance . def . requires_caller_location (machine . tcx)) ; stacktrace . truncate (1) ; (stacktrace , false) } BacktraceStyle :: Short => { let original_len = stacktrace . len () ; let has_local_frame = stacktrace . iter () . any (| frame | machine . is_local (frame)) ; if has_local_frame { stacktrace . retain (| frame | ! frame . instance . def . requires_caller_location (machine . tcx)) ; stacktrace = stacktrace . into_iter () . take_while (| frame | { let def_id = frame . instance . def_id () ; let path = machine . tcx . def_path_str (def_id) ; ! path . contains ("__rust_begin_short_backtrace") }) . collect :: < Vec < _ > > () ; while stacktrace . len () > 1 && stacktrace . last () . is_some_and (| frame | ! machine . is_local (frame)) { stacktrace . pop () ; } } let was_pruned = stacktrace . len () != original_len ; (stacktrace , was_pruned) } BacktraceStyle :: Full => (stacktrace , false) , } }
};
}
