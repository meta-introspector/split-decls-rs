// Generated macro for is_str_and_string (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedis_str_and_string {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"is_str_and_string"}
// Dependencies: {}
fn is_str_and_string (cx : & LateContext < '_ > , arg_ty : Ty < '_ > , original_arg_ty : Ty < '_ >) -> bool { original_arg_ty . is_str () && arg_ty . is_lang_item (cx , LangItem :: String) }
};
}
