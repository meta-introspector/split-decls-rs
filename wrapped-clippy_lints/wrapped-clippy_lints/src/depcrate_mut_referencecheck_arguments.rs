// Generated macro for check_arguments (function)
macro_rules! Depcrate_mut_referencecheck_arguments {
() => {
// Module: crate::mut_reference
// Provides: {"check_arguments"}
// Dependencies: {}
fn check_arguments < 'tcx > (cx : & LateContext < 'tcx > , arguments : & mut dyn Iterator < Item = & 'tcx Expr < 'tcx > > , type_definition : Ty < 'tcx > , name : & str , fn_kind : & str ,) { if type_definition . is_fn () { let parameters = type_definition . fn_sig (cx . tcx) . skip_binder () . inputs () ; for (argument , parameter) in iter :: zip (arguments , parameters) { if let ty :: Ref (_ , _ , Mutability :: Not) | ty :: RawPtr (_ , Mutability :: Not) = parameter . kind () && let ExprKind :: AddrOf (BorrowKind :: Ref , Mutability :: Mut , arg) = argument . kind { let mut applicability = Applicability :: MachineApplicable ; let sugg = Sugg :: hir_with_applicability (cx , arg , "_" , & mut applicability) . addr () ; span_lint_and_sugg (cx , UNNECESSARY_MUT_PASSED , argument . span , format ! ("the {fn_kind} `{name}` doesn't need a mutable reference") , "remove this `mut`" , sugg . to_string () , applicability ,) ; } } } }
};
}
