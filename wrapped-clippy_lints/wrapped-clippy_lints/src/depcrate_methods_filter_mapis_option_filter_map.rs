// Generated macro for is_option_filter_map (function)
macro_rules! Depcrate_methods_filter_mapis_option_filter_map {
() => {
// Module: crate::methods::filter_map
// Provides: {"is_option_filter_map"}
// Dependencies: {}
fn is_option_filter_map (cx : & LateContext < '_ > , filter_arg : & Expr < '_ > , map_arg : & Expr < '_ >) -> bool { is_method (cx , map_arg , sym :: unwrap) && is_method (cx , filter_arg , sym :: is_some) }
};
}
