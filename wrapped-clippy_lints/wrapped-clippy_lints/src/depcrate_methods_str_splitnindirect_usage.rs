// Generated macro for indirect_usage (function)
macro_rules! Depcrate_methods_str_splitnindirect_usage {
() => {
// Module: crate::methods::str_splitn
// Provides: {"indirect_usage"}
// Dependencies: {}
# [doc = " returns `Some(IndirectUsage)` for e.g."] # [doc = ""] # [doc = " ```ignore"] # [doc = " let name = binding.next()?;"] # [doc = " let name = binding.next().unwrap();"] # [doc = " ```"] fn indirect_usage < 'tcx > (cx : & LateContext < 'tcx > , stmt : & Stmt < 'tcx > , binding : HirId , ctxt : SyntaxContext ,) -> Option < IndirectUsage < 'tcx > > { if let StmtKind :: Let (& LetStmt { pat : Pat { kind : PatKind :: Binding (BindingMode :: NONE , _ , ident , None) , .. } , init : Some (init_expr) , hir_id : local_hir_id , .. }) = stmt . kind { let mut path_to_binding = None ; let _ : Option < ! > = for_each_expr (cx , init_expr , | e | { if e . res_local_id () == Some (binding) { path_to_binding = Some (e) ; } ControlFlow :: Continue (Descend :: from (path_to_binding . is_none ())) }) ; let mut parents = cx . tcx . hir_parent_iter (path_to_binding ? . hir_id) ; let iter_usage = parse_iter_usage (cx , ctxt , & mut parents) ? ; let (parent_id , _) = parents . find (| (_ , node) | { ! matches ! (node , Node :: Expr (Expr { kind : ExprKind :: Match (.., MatchSource :: TryDesugar (_)) , .. })) }) ? ; if let IterUsage { kind : IterUsageKind :: Nth (0) , unwrap_kind : Some (unwrap_kind) , .. } = iter_usage && parent_id == local_hir_id { return Some (IndirectUsage { name : ident . name , span : stmt . span , init_expr , unwrap_kind , }) ; } } None }
};
}
