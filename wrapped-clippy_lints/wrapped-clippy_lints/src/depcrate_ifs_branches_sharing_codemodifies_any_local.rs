// Generated macro for modifies_any_local (function)
macro_rules! Depcrate_ifs_branches_sharing_codemodifies_any_local {
() => {
// Module: crate::ifs::branches_sharing_code
// Provides: {"modifies_any_local"}
// Dependencies: {}
# [doc = " Checks if the statement modifies or moves any of the given locals."] fn modifies_any_local < 'tcx > (cx : & LateContext < 'tcx > , s : & 'tcx Stmt < '_ > , locals : & HirIdSet) -> bool { for_each_expr_without_closures (s , | e | { if let Some (id) = e . res_local_id () && locals . contains (& id) && ! capture_local_usage (cx , e) . is_imm_ref () { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } }) . is_some () }
};
}
