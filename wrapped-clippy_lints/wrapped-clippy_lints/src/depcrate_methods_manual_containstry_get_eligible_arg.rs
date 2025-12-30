// Generated macro for try_get_eligible_arg (function)
macro_rules! Depcrate_methods_manual_containstry_get_eligible_arg {
() => {
// Module: crate::methods::manual_contains
// Provides: {"try_get_eligible_arg"}
// Dependencies: {}
fn try_get_eligible_arg < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , closure_arg_id : HirId , applicability : & mut Applicability ,) -> Option < (EligibleArg , & 'tcx Expr < 'tcx >) > { let mut get_snippet = | expr : & Expr < '_ > , needs_borrow : bool | { let sugg = Sugg :: hir_with_applicability (cx , expr , "_" , applicability) ; EligibleArg :: ContainsArg ((if needs_borrow { sugg . addr () } else { sugg }) . to_string ()) } ; match expr . kind { ExprKind :: Path (QPath :: Resolved (_ , path)) => { if path . res == Res :: Local (closure_arg_id) { Some ((EligibleArg :: IsClosureArg , expr)) } else { Some ((get_snippet (expr , true) , expr)) } } , ExprKind :: Unary (UnOp :: Deref , inner) => { if let ExprKind :: Path (QPath :: Resolved (_ , path)) = inner . kind { if path . res == Res :: Local (closure_arg_id) { Some ((EligibleArg :: IsClosureArg , expr)) } else { Some ((get_snippet (inner , false) , expr)) } } else { None } } , _ => { if switch_to_eager_eval (cx , expr) { Some ((get_snippet (expr , true) , expr)) } else { None } } , } }
};
}
