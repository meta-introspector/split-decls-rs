// Generated macro for local_used_in (function)
macro_rules! Depcrate_usagelocal_used_in {
() => {
// Module: crate::usage
// Provides: {"local_used_in"}
// Dependencies: {}
pub fn local_used_in < 'tcx > (cx : & LateContext < 'tcx > , local_id : HirId , v : impl Visitable < 'tcx >) -> bool { for_each_expr (cx , v , | e | { if e . res_local_id () == Some (local_id) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } }) . is_some () }
};
}
