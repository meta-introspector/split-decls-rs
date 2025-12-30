// Generated macro for local_used_once (function)
macro_rules! Depcrate_visitorslocal_used_once {
() => {
// Module: crate::visitors
// Provides: {"local_used_once"}
// Dependencies: {}
# [doc = " If the local is only used once in `visitable` returns the path expression referencing the given"] # [doc = " local"] pub fn local_used_once < 'tcx > (cx : & LateContext < 'tcx > , visitable : impl Visitable < 'tcx > , id : HirId ,) -> Option < & 'tcx Expr < 'tcx > > { let mut expr = None ; let cf = for_each_expr (cx , visitable , | e | { if e . res_local_id () == Some (id) && expr . replace (e) . is_some () { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } }) ; if cf . is_some () { return None ; } expr }
};
}
