// Generated macro for is_c_str_function (function)
macro_rules! Depcrate_methods_manual_c_str_literalsis_c_str_function {
() => {
// Module: crate::methods::manual_c_str_literals
// Provides: {"is_c_str_function"}
// Dependencies: {}
# [doc = " Checks if the callee is a \"relevant\" `CStr` function considered by this lint."] # [doc = " Returns the function name."] fn is_c_str_function (cx : & LateContext < '_ > , func : & Expr < '_ >) -> Option < Symbol > { if let ExprKind :: Path (QPath :: TypeRelative (cstr , fn_name)) = & func . kind && let TyKind :: Path (QPath :: Resolved (_ , ty_path)) = & cstr . kind && cx . tcx . lang_items () . c_str () == ty_path . res . opt_def_id () { Some (fn_name . ident . name) } else { None } }
};
}
