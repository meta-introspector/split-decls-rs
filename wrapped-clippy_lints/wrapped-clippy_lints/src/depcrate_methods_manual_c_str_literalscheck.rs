// Generated macro for check (function)
macro_rules! Depcrate_methods_manual_c_str_literalscheck {
() => {
// Module: crate::methods::manual_c_str_literals
// Provides: {"check"}
// Dependencies: {}
# [doc = " Checks calls to the `CStr` constructor functions:"] # [doc = " - `CStr::from_bytes_with_nul(..)`"] # [doc = " - `CStr::from_bytes_with_nul_unchecked(..)`"] # [doc = " - `CStr::from_ptr(..)`"] pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , func : & Expr < '_ > , args : & [Expr < '_ >] , msrv : Msrv) { if let Some (fn_name) = is_c_str_function (cx , func) && let [arg] = args && cx . tcx . sess . edition () >= Edition2021 && msrv . meets (cx , msrvs :: C_STR_LITERALS) { match fn_name { sym :: from_bytes_with_nul | sym :: from_bytes_with_nul_unchecked if ! arg . span . from_expansion () && let ExprKind :: Lit (lit) = arg . kind && let LitKind :: ByteStr (_ , StrStyle :: Cooked) | LitKind :: Str (_ , StrStyle :: Cooked) = lit . node => { check_from_bytes (cx , expr , arg , fn_name) ; } , sym :: from_ptr => check_from_ptr (cx , expr , arg) , _ => { } , } } }
};
}
