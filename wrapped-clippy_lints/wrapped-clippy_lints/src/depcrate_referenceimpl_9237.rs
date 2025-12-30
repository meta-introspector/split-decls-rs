// Generated macro for impl_9237 (impl)
macro_rules! Depcrate_referenceimpl_9237 {
() => {
// Module: crate::reference
// Provides: {"impl_9237"}
// Dependencies: {}
impl LateLintPass < '_ > for DerefAddrOf { fn check_expr (& mut self , cx : & LateContext < '_ > , e : & Expr < '_ >) { if ! e . span . from_expansion () && let ExprKind :: Unary (UnOp :: Deref , deref_target) = e . kind && ! deref_target . span . from_expansion () && let ExprKind :: AddrOf (_ , _ , addrof_target) = deref_target . kind && ! matches ! (addrof_target . kind , ExprKind :: Array (_)) && deref_target . span . eq_ctxt (e . span) && ! addrof_target . span . from_expansion () { let mut applicability = Applicability :: MachineApplicable ; let mut sugg = | | Sugg :: hir_with_applicability (cx , addrof_target , "_" , & mut applicability) ; let sugg = match is_manually_drop_through_union (cx , e . hir_id , addrof_target) { ManuallyDropThroughUnion :: Directly => sugg () . deref () , ManuallyDropThroughUnion :: Indirect => return , ManuallyDropThroughUnion :: No => sugg () , } ; let sugg = if has_enclosing_paren (snippet (cx , e . span , "")) { sugg . maybe_paren () } else { sugg } ; span_lint_and_sugg (cx , DEREF_ADDROF , e . span , "immediately dereferencing a reference" , "try" , sugg . to_string () , applicability ,) ; } } }
};
}
