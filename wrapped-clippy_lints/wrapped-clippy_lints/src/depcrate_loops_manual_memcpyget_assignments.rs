// Generated macro for get_assignments (function)
macro_rules! Depcrate_loops_manual_memcpyget_assignments {
() => {
// Module: crate::loops::manual_memcpy
// Provides: {"get_assignments"}
// Dependencies: {}
# [doc = " Get assignments from the given block."] # [doc = " The returned iterator yields `None` if no assignment expressions are there,"] # [doc = " filtering out the increments of the given whitelisted loop counters;"] # [doc = " because its job is to make sure there's nothing other than assignments and the increments."] fn get_assignments < 'a , 'tcx > (Block { stmts , expr , .. } : & 'tcx Block < 'tcx > , loop_counters : & 'a [Start < 'tcx >] ,) -> impl Iterator < Item = Option < (& 'tcx Expr < 'tcx > , & 'tcx Expr < 'tcx >) > > + 'a { stmts . iter () . filter_map (move | stmt | match stmt . kind { StmtKind :: Let (..) | StmtKind :: Item (..) => None , StmtKind :: Expr (e) | StmtKind :: Semi (e) => Some (e) , }) . chain (* expr) . filter (move | e | { if let ExprKind :: AssignOp (_ , place , _) = e . kind { place . res_local_id () . is_some_and (| id | { ! loop_counters . iter () . skip (1) . any (| counter | counter . id == id) }) } else { true } }) . map (get_assignment) }
};
}
