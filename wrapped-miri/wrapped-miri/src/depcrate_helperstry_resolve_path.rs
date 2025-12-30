// Generated macro for try_resolve_path (function)
macro_rules! Depcrate_helperstry_resolve_path {
() => {
// Module: crate::helpers
// Provides: {"try_resolve_path"}
// Dependencies: {}
# [doc = " Gets an instance for a path; fails gracefully if the path does not exist."] pub fn try_resolve_path < 'tcx > (tcx : TyCtxt < 'tcx > , path : & [& str] , namespace : Namespace ,) -> Option < ty :: Instance < 'tcx > > { let did = try_resolve_did (tcx , path , Some (namespace)) ? ; Some (ty :: Instance :: mono (tcx , did)) }
};
}
