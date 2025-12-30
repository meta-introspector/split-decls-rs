// Generated macro for check_string (function)
macro_rules! Depcrate_methods_drain_collectcheck_string {
() => {
// Module: crate::methods::drain_collect
// Provides: {"check_string"}
// Dependencies: {}
# [doc = " Checks `std::string::String`"] fn check_string (cx : & LateContext < '_ > , args : & [Expr < '_ >] , expr : Ty < '_ > , recv : Ty < '_ > , recv_path : & Path < '_ >) -> bool { expr . is_lang_item (cx , LangItem :: String) && recv . is_lang_item (cx , LangItem :: String) && matches ! (args , [arg] if is_range_full (cx , arg , Some (recv_path))) }
};
}
