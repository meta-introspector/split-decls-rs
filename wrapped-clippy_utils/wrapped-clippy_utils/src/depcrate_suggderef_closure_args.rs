// Generated macro for deref_closure_args (function)
macro_rules! Depcrate_suggderef_closure_args {
() => {
// Module: crate::sugg
// Provides: {"deref_closure_args"}
// Dependencies: {}
# [doc = " Build suggestion gradually by handling closure arg specific usages,"] # [doc = " such as explicit deref and borrowing cases."] # [doc = " Returns `None` if no such use cases have been triggered in closure body"] # [doc = ""] # [doc = " note: This only works on immutable closures with exactly one input parameter."] pub fn deref_closure_args (cx : & LateContext < '_ > , closure : & hir :: Expr < '_ >) -> Option < DerefClosure > { if let ExprKind :: Closure (& Closure { fn_decl , def_id , body , .. }) = closure . kind { let closure_body = cx . tcx . hir_body (body) ; let closure_arg_is_type_annotated_double_ref = if let TyKind :: Ref (_ , MutTy { ty , .. }) = fn_decl . inputs [0] . kind { matches ! (ty . kind , TyKind :: Ref (_ , MutTy { .. })) } else { false } ; let mut visitor = DerefDelegate { cx , closure_span : closure . span , closure_arg_id : closure_body . params [0] . pat . hir_id , closure_arg_is_type_annotated_double_ref , next_pos : closure . span . lo () , checked_borrows : FxHashSet :: default () , suggestion_start : String :: new () , applicability : Applicability :: MachineApplicable , } ; ExprUseVisitor :: for_clippy (cx , def_id , & mut visitor) . consume_body (closure_body) . into_ok () ; if ! visitor . suggestion_start . is_empty () { return Some (DerefClosure { applicability : visitor . applicability , suggestion : visitor . finish () , }) ; } } None }
};
}
