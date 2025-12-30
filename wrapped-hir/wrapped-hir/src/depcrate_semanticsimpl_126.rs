// Generated macro for impl_126 (impl)
macro_rules! Depcrate_semanticsimpl_126 {
() => {
// Module: crate::semantics
// Provides: {"impl_126"}
// Dependencies: {}
impl RenameConflictsVisitor < '_ > { fn resolve_path (& mut self , node : ExprOrPatId , path : & Path) { if let Path :: BarePath (path) = path && let Some (name) = path . as_ident () { if * name . symbol () == self . new_name { if let Some (conflicting) = self . resolver . rename_will_conflict_with_renamed (self . db , name , path , self . body . expr_or_pat_path_hygiene (node) , self . to_be_renamed ,) { self . conflicts . insert (conflicting) ; } } else if * name . symbol () == self . old_name && let Some (conflicting) = self . resolver . rename_will_conflict_with_another_variable (self . db , name , path , self . body . expr_or_pat_path_hygiene (node) , & self . new_name , self . to_be_renamed ,) { self . conflicts . insert (conflicting) ; } } } fn rename_conflicts (& mut self , expr : ExprId) { match & self . body [expr] { Expr :: Path (path) => { let guard = self . resolver . update_to_inner_scope (self . db , self . owner , expr) ; self . resolve_path (expr . into () , path) ; self . resolver . reset_to_guard (guard) ; } & Expr :: Assignment { target , .. } => { let guard = self . resolver . update_to_inner_scope (self . db , self . owner , expr) ; self . body . walk_pats (target , & mut | pat | { if let Pat :: Path (path) = & self . body [pat] { self . resolve_path (pat . into () , path) ; } }) ; self . resolver . reset_to_guard (guard) ; } _ => { } } self . body . walk_child_exprs (expr , | expr | self . rename_conflicts (expr)) ; } }
};
}
