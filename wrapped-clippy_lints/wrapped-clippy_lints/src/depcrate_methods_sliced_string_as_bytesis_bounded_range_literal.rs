// Generated macro for is_bounded_range_literal (function)
macro_rules! Depcrate_methods_sliced_string_as_bytesis_bounded_range_literal {
() => {
// Module: crate::methods::sliced_string_as_bytes
// Provides: {"is_bounded_range_literal"}
// Dependencies: {}
# [doc = " Checks if `index` is any type of range except `RangeFull` (i.e. `..`)"] fn is_bounded_range_literal (cx : & LateContext < '_ > , index : & Expr < '_ >) -> bool { higher :: Range :: hir (cx , index) . is_some_and (| range | Option :: or (range . start , range . end) . is_some ()) }
};
}
