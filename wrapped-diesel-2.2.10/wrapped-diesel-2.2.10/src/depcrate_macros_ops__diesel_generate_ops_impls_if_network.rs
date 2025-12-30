// Generated macro for __diesel_generate_ops_impls_if_network (macro)
macro_rules! Depcrate_macros_ops__diesel_generate_ops_impls_if_network {
() => {
// Module: crate::macros::ops
// Provides: {"__diesel_generate_ops_impls_if_network"}
// Dependencies: {}
# [macro_export (local_inner_macros)] # [doc (hidden)] macro_rules ! __diesel_generate_ops_impls_if_network { ($ column_name : ident , Nullable <$ ($ inner : tt) ::*>) => { __diesel_generate_ops_impls_if_network ! ($ column_name , $ ($ inner) ::*) ; } ; ($ column_name : ident , Cidr) => { network_expr ! ($ column_name) ; } ; ($ column_name : ident , Inet) => { network_expr ! ($ column_name) ; } ; ($ column_name : ident , $ non_network_type : ty) => { } ; }
};
}
