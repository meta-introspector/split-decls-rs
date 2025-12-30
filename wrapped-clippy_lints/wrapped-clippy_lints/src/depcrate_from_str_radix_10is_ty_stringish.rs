// Generated macro for is_ty_stringish (function)
macro_rules! Depcrate_from_str_radix_10is_ty_stringish {
() => {
// Module: crate::from_str_radix_10
// Provides: {"is_ty_stringish"}
// Dependencies: {}
# [doc = " Checks if a Ty is `String` or `&str`"] fn is_ty_stringish (cx : & LateContext < '_ > , ty : Ty < '_ >) -> bool { ty . is_lang_item (cx , LangItem :: String) || ty . peel_refs () . is_str () }
};
}
