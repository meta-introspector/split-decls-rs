// Generated macro for eq_stmts (function)
macro_rules! Depcrate_ifs_branches_sharing_codeeq_stmts {
() => {
// Module: crate::ifs::branches_sharing_code
// Provides: {"eq_stmts"}
// Dependencies: {}
# [doc = " Checks if the given statement should be considered equal to the statement in the same"] # [doc = " position for each block."] fn eq_stmts (stmt : & Stmt < '_ > , blocks : & [& Block < '_ >] , get_stmt : impl for < 'a > Fn (& 'a Block < 'a >) -> Option < & 'a Stmt < 'a > > , eq : & mut HirEqInterExpr < '_ , '_ , '_ > , moved_bindings : & mut Vec < (HirId , Symbol) > ,) -> bool { (if let StmtKind :: Let (l) = stmt . kind { let old_count = moved_bindings . len () ; l . pat . each_binding_or_first (& mut | _ , id , _ , name | { moved_bindings . push ((id , name . name)) ; }) ; let new_bindings = & moved_bindings [old_count ..] ; blocks . iter () . all (| b | get_stmt (b) . is_some_and (| s | eq_binding_names (s , new_bindings))) } else { true }) && blocks . iter () . all (| b | get_stmt (b) . is_some_and (| s | eq . eq_stmt (s , stmt))) }
};
}
