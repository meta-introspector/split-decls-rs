// Generated macro for check_fn_sig (function)
macro_rules! Depcrate_functions_ref_optioncheck_fn_sig {
() => {
// Module: crate::functions::ref_option
// Provides: {"check_fn_sig"}
// Dependencies: {}
fn check_fn_sig < 'a > (cx : & LateContext < 'a > , decl : & FnDecl < 'a > , span : Span , sig : ty :: FnSig < 'a >) { let mut fixes = Vec :: new () ; for (param , param_ty) in decl . inputs . iter () . zip (sig . inputs ()) { check_ty (cx , param , * param_ty , & mut fixes) ; } if let hir :: FnRetTy :: Return (ty) = & decl . output { check_ty (cx , ty , sig . output () , & mut fixes) ; } if ! fixes . is_empty () { span_lint_and_then (cx , REF_OPTION , span , "it is more idiomatic to use `Option<&T>` instead of `&Option<T>`" , | diag | { diag . multipart_suggestion ("change this to" , fixes , Applicability :: Unspecified) ; } ,) ; } }
};
}
