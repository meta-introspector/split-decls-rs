// Generated macro for field_reassigned_by_stmt (function)
macro_rules! Depcrate_defaultfield_reassigned_by_stmt {
() => {
// Module: crate::default
// Provides: {"field_reassigned_by_stmt"}
// Dependencies: {}
# [doc = " Returns the reassigned field and the assigning expression (right-hand side of assign)."] fn field_reassigned_by_stmt < 'tcx > (this : & Stmt < 'tcx > , binding_name : Symbol) -> Option < (Ident , & 'tcx Expr < 'tcx >) > { if let StmtKind :: Semi (later_expr) = this . kind && let ExprKind :: Assign (assign_lhs , assign_rhs , _) = later_expr . kind && let ExprKind :: Field (binding , field_ident) = assign_lhs . kind && let ExprKind :: Path (QPath :: Resolved (_ , path)) = binding . kind && let Some (second_binding_name) = path . segments . last () && second_binding_name . ident . name == binding_name { Some ((field_ident , assign_rhs)) } else { None } }
};
}
