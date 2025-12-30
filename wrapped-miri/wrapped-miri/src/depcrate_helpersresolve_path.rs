// Generated macro for resolve_path (function)
macro_rules! Depcrate_helpersresolve_path {
() => {
// Module: crate::helpers
// Provides: {"resolve_path"}
// Dependencies: {}
# [doc = " Gets an instance for a path."] # [track_caller] pub fn resolve_path < 'tcx > (tcx : TyCtxt < 'tcx > , path : & [& str] , namespace : Namespace ,) -> ty :: Instance < 'tcx > { try_resolve_path (tcx , path , namespace) . unwrap_or_else (| | panic ! ("failed to find required Rust item: {path:?}")) }
};
}
