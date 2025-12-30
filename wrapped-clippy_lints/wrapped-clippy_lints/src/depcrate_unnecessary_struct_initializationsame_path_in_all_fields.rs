// Generated macro for same_path_in_all_fields (function)
macro_rules! Depcrate_unnecessary_struct_initializationsame_path_in_all_fields {
() => {
// Module: crate::unnecessary_struct_initialization
// Provides: {"same_path_in_all_fields"}
// Dependencies: {}
# [doc = " Check whether all fields of a struct assignment match."] # [doc = " Returns a [Path] item that one can obtain a span from for the lint suggestion."] # [doc = ""] # [doc = " Conditions that must be satisfied to trigger this variant of the lint:"] # [doc = ""] # [doc = " - source struct of the assignment must be of same type as the destination"] # [doc = " - names of destination struct fields must match the field names of the source"] # [doc = ""] # [doc = " We don’t check here if all struct fields are assigned as the remainder may"] # [doc = " be filled in from a base struct."] fn same_path_in_all_fields < 'tcx > (cx : & LateContext < '_ > , expr : & Expr < '_ > , fields : & [ExprField < 'tcx >] ,) -> Option < & 'tcx Path < 'tcx > > { let ty = cx . typeck_results () . expr_ty (expr) ; let mut found = None ; for f in fields { if let ExprKind :: Field (src_expr , ident) = f . expr . kind && ty == cx . typeck_results () . expr_ty (src_expr) && ident_without_range_desugaring (f . ident) == ident && let ExprKind :: Path (QPath :: Resolved (None , src_path)) = src_expr . kind { let Some ((_ , p)) = found else { found = Some ((src_expr , src_path)) ; continue ; } ; if p . res == src_path . res { continue ; } } return None ; } if let Some ((src_expr , src_path)) = found && check_references (cx , expr , src_expr) { Some (src_path) } else { None } }
};
}
