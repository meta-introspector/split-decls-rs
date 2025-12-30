// Generated macro for __diesel_generate_ops_impls_if_date_time (macro)
macro_rules! Depcrate_macros_ops__diesel_generate_ops_impls_if_date_time {
() => {
// Module: crate::macros::ops
// Provides: {"__diesel_generate_ops_impls_if_date_time"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! __diesel_generate_ops_impls_if_date_time { ($ column_name : ident , Nullable <$ ($ inner : tt) ::*>) => { $ crate :: __diesel_generate_ops_impls_if_date_time ! ($ column_name , $ ($ inner) ::*) ; } ; ($ column_name : ident , Time) => { $ crate :: date_time_expr ! ($ column_name) ; } ; ($ column_name : ident , Date) => { $ crate :: date_time_expr ! ($ column_name) ; } ; ($ column_name : ident , Timestamp) => { $ crate :: date_time_expr ! ($ column_name) ; } ; ($ column_name : ident , Timestamptz) => { $ crate :: date_time_expr ! ($ column_name) ; } ; ($ column_name : ident , $ non_date_time_type : ty) => { } ; }
};
}
