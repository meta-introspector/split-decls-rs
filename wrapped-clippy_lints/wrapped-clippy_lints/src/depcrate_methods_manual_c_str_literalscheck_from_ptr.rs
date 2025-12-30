// Generated macro for check_from_ptr (function)
macro_rules! Depcrate_methods_manual_c_str_literalscheck_from_ptr {
() => {
// Module: crate::methods::manual_c_str_literals
// Provides: {"check_from_ptr"}
// Dependencies: {}
# [doc = " Checks `CStr::from_ptr(b\"foo\\0\".as_ptr().cast())`"] fn check_from_ptr (cx : & LateContext < '_ > , expr : & Expr < '_ > , arg : & Expr < '_ >) { if let ExprKind :: MethodCall (method , lit , [] , _) = peel_ptr_cast (arg) . kind && method . ident . name == sym :: as_ptr && ! lit . span . from_expansion () && let ExprKind :: Lit (lit) = lit . kind && let LitKind :: ByteStr (_ , StrStyle :: Cooked) = lit . node && let Some (sugg) = rewrite_as_cstr (cx , lit . span) { span_lint_and_sugg (cx , MANUAL_C_STR_LITERALS , expr . span , "calling `CStr::from_ptr` with a byte string literal" , r#"use a `c""` literal"# , sugg , Applicability :: MachineApplicable ,) ; } }
};
}
