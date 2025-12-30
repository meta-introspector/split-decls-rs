// Generated macro for check_expr_for_enum_as_function (function)
macro_rules! Depcrate_empty_with_bracketscheck_expr_for_enum_as_function {
() => {
// Module: crate::empty_with_brackets
// Provides: {"check_expr_for_enum_as_function"}
// Dependencies: {}
fn check_expr_for_enum_as_function (expr : & Expr < '_ >) -> Option < LocalDefId > { if let ExprKind :: Path (QPath :: Resolved (_ , Path { res : Def (Ctor (CtorOf :: Variant , _) , def_id) , .. } ,)) = expr . kind { def_id . as_local () } else { None } }
};
}
