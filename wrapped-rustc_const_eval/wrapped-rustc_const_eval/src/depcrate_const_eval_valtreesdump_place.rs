// Generated macro for dump_place (function)
macro_rules! Depcrate_const_eval_valtreesdump_place {
() => {
// Module: crate::const_eval::valtrees
// Provides: {"dump_place"}
// Dependencies: {}
fn dump_place < 'tcx > (ecx : & CompileTimeInterpCx < 'tcx > , place : & MPlaceTy < 'tcx >) { trace ! ("{:?}" , ecx . dump_place (& PlaceTy :: from (place . clone ()))) ; }
};
}
