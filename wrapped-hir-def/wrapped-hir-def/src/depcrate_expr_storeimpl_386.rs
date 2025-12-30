// Generated macro for impl_386 (impl)
macro_rules! Depcrate_expr_storeimpl_386 {
() => {
// Module: crate::expr_store
// Provides: {"impl_386"}
// Dependencies: {}
impl PartialEq for ExpressionStoreSourceMap { fn eq (& self , other : & Self) -> bool { let Self { expr_only , types_map_back , types_map : _ , lifetime_map_back , lifetime_map : _ } = self ; * expr_only == other . expr_only && * types_map_back == other . types_map_back && * lifetime_map_back == other . lifetime_map_back } }
};
}
