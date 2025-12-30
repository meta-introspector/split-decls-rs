// Generated macro for track_span_parent (function)
macro_rules! Depcrate_callbackstrack_span_parent {
() => {
// Module: crate::callbacks
// Provides: {"track_span_parent"}
// Dependencies: {}
fn track_span_parent (def_id : rustc_span :: def_id :: LocalDefId) { tls :: with_context_opt (| icx | { if let Some (icx) = icx { let tracks_deps = match icx . task_deps { TaskDepsRef :: Allow (..) => true , TaskDepsRef :: EvalAlways | TaskDepsRef :: Ignore | TaskDepsRef :: Forbid => false , } ; if tracks_deps { let _span = icx . tcx . source_span (def_id) ; debug_assert_eq ! (_span . data_untracked () . parent , None) ; } } }) }
};
}
