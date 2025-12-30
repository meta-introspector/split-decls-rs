// Generated macro for __internal_alias_helper (macro)
macro_rules! Depcrate_query_source_aliasing_macros__internal_alias_helper {
() => {
// Module: crate::query_source::aliasing::macros
// Provides: {"__internal_alias_helper"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] # [doc = " This only exists to hide internals from the doc"] macro_rules ! __internal_alias_helper { (table_ty = $ left_table_ty : ty , table_tt = $ left_table_tt : tt , alias_ty = $ left_alias : ident , alias_sql_name = $ left_sql_name : ident ; $ (table_ty = $ right_table_ty : ty , table_tt = $ right_table_tt : tt , alias_ty = $ right_alias : ident , alias_sql_name = $ right_sql_name : ident ;) +) => { $ ($ crate :: static_cond ! { if ($ left_table_tt) == ($ right_table_tt) { $ crate :: static_cond ! { if ($ left_sql_name) != ($ right_sql_name) { impl $ crate :: internal :: alias_macro :: AliasAliasAppearsInFromClauseSameTable <$ left_alias , $ left_table_ty > for $ right_alias { type Count = $ crate :: query_source :: Never ; } impl $ crate :: internal :: alias_macro :: AliasAliasAppearsInFromClauseSameTable <$ right_alias , $ left_table_ty > for $ left_alias { type Count = $ crate :: query_source :: Never ; } } } } }) * $ crate :: __internal_alias_helper ! ($ (table_ty = $ right_table_ty , table_tt = $ right_table_tt , alias_ty = $ right_alias , alias_sql_name = $ right_sql_name ;) +) ; } ; (table_ty = $ left_table_ty : ty , table_tt = $ left_table_tt : tt , alias_ty = $ left_alias : ident , alias_sql_name = $ left_sql_name : ident ;) => { } ; () => { } ; }
};
}
