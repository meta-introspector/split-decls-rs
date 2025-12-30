// Generated macro for check_arguments (function)
macro_rules! Depcrate_unnecessary_mut_passedcheck_arguments {
() => {
// Module: crate::unnecessary_mut_passed
// Provides: {"check_arguments"}
// Dependencies: {}
fn check_arguments < 'tcx > (cx : & LateContext < 'tcx > , arguments : & mut dyn Iterator < Item = & 'tcx Expr < 'tcx > > , type_definition : Ty < 'tcx > , name : & str , fn_kind : & str ,) { if type_definition . is_fn () { let parameters = type_definition . fn_sig (cx . tcx) . skip_binder () . inputs () ; for (argument , parameter) in iter :: zip (arguments , parameters) { if let ty :: Ref (_ , _ , Mutability :: Not) | ty :: RawPtr (_ , Mutability :: Not) = parameter . kind () && let ExprKind :: AddrOf (BorrowKind :: Ref , Mutability :: Mut , arg) = argument . kind { let applicability = Applicability :: MachineApplicable ; let span_to_remove = { let span_until_arg = argument . span . until (arg . span) ; if let Some (Some (ref_pos)) = span_until_arg . with_source_text (cx , | src | { src . find ('&') . filter (| ref_pos | src [* ref_pos ..] . contains ("mut")) }) && let Ok (lo) = u32 :: try_from (ref_pos + '&' . len_utf8 ()) { span_until_arg . split_at (lo) . 1 } else { return ; } } ; span_lint_and_then (cx , UNNECESSARY_MUT_PASSED , argument . span , format ! ("the {fn_kind} `{name}` doesn't need a mutable reference") , | diag | { diag . span_suggestion_verbose (span_to_remove , "remove this `mut`" , String :: new () , applicability) ; } ,) ; } } } }
};
}
