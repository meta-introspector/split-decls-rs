// Generated macro for emit_lint_assign (function)
macro_rules! Depcrate_methods_swap_with_temporaryemit_lint_assign {
() => {
// Module: crate::methods::swap_with_temporary
// Provides: {"emit_lint_assign"}
// Dependencies: {}
fn emit_lint_assign (cx : & LateContext < '_ > , expr : & Expr < '_ > , target : & ArgKind < '_ > , reftemp : & Expr < '_ > , temp : & Expr < '_ >) { span_lint_and_then (cx , SWAP_WITH_TEMPORARY , expr . span , "swapping with a temporary value is inefficient" , | diag | { if ! emit_note (diag , expr , reftemp , temp) { return ; } if matches ! (cx . tcx . parent_hir_node (expr . hir_id) , Node :: Stmt (..) | Node :: Block (..)) { let mut applicability = Applicability :: MachineApplicable ; let ctxt = expr . span . ctxt () ; let assign_target = match target { ArgKind :: Expr (target) => Sugg :: hir_with_context (cx , target , ctxt , "_" , & mut applicability) . deref () , ArgKind :: RefMutToPlaceAsMacro (arg , derefs) => (0 .. * derefs) . fold (Sugg :: hir_with_context (cx , arg , ctxt , "_" , & mut applicability) . deref () , | sugg , _ | sugg . deref () ,) , ArgKind :: RefMutToPlace (target , derefs) => (0 .. * derefs) . fold (Sugg :: hir_with_context (cx , target , ctxt , "_" , & mut applicability) , | sugg , _ | sugg . deref () ,) , ArgKind :: RefMutToTemp (_) => unreachable ! () , } ; let assign_source = Sugg :: hir_with_context (cx , temp , ctxt , "_" , & mut applicability) ; diag . span_suggestion (expr . span , "use assignment instead" , format ! ("{assign_target} = {assign_source}") , applicability ,) ; } } ,) ; }
};
}
