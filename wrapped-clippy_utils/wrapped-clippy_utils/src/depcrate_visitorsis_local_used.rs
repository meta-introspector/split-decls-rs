// Generated macro for is_local_used (function)
macro_rules! Depcrate_visitorsis_local_used {
() => {
// Module: crate::visitors
// Provides: {"is_local_used"}
// Dependencies: {}
# [doc = " Checks if the given local is used."] pub fn is_local_used < 'tcx > (cx : & LateContext < 'tcx > , visitable : impl Visitable < 'tcx > , id : HirId) -> bool { for_each_expr (cx , visitable , | e | { if e . res_local_id () == Some (id) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } }) . is_some () }
};
}
