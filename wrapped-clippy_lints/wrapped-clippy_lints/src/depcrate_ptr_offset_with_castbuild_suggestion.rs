// Generated macro for build_suggestion (function)
macro_rules! Depcrate_ptr_offset_with_castbuild_suggestion {
() => {
// Module: crate::ptr_offset_with_cast
// Provides: {"build_suggestion"}
// Dependencies: {}
fn build_suggestion (cx : & LateContext < '_ > , method : Method , receiver_expr : & Expr < '_ > , cast_lhs_expr : & Expr < '_ > ,) -> Option < String > { let receiver = receiver_expr . span . get_source_text (cx) ? ; let cast_lhs = cast_lhs_expr . span . get_source_text (cx) ? ; Some (format ! ("{receiver}.{}({cast_lhs})" , method . suggestion ())) }
};
}
