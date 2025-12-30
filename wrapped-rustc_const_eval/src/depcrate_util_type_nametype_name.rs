// Generated macro for type_name (function)
macro_rules! Depcrate_util_type_nametype_name {
() => {
// Module: crate::util::type_name
// Provides: {"type_name"}
// Dependencies: {}
pub fn type_name < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> String { let mut p = TypeNamePrinter { tcx , path : String :: new () } ; p . print_type (ty) . unwrap () ; p . path }
};
}
