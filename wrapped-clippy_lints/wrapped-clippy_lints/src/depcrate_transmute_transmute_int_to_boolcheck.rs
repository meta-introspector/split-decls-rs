// Generated macro for check (function)
macro_rules! Depcrate_transmute_transmute_int_to_boolcheck {
() => {
// Module: crate::transmute::transmute_int_to_bool
// Provides: {"check"}
// Dependencies: {}
# [doc = " Checks for `transmute_int_to_bool` lint."] # [doc = " Returns `true` if it's triggered, otherwise returns `false`."] pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ > , from_ty : Ty < 'tcx > , to_ty : Ty < 'tcx > , arg : & 'tcx Expr < '_ > ,) -> bool { match (& from_ty . kind () , & to_ty . kind ()) { (ty :: Int (ty :: IntTy :: I8) | ty :: Uint (ty :: UintTy :: U8) , ty :: Bool) => { span_lint_and_then (cx , TRANSMUTE_INT_TO_BOOL , e . span , format ! ("transmute from a `{from_ty}` to a `bool`") , | diag | { let arg = sugg :: Sugg :: hir (cx , arg , "..") ; let zero = sugg :: Sugg :: NonParen (Cow :: from ("0")) ; diag . span_suggestion (e . span , "consider using" , sugg :: make_binop (ast :: BinOpKind :: Ne , & arg , & zero) . to_string () , Applicability :: Unspecified ,) ; } ,) ; true } , _ => false , } }
};
}
