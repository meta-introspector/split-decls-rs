// Generated macro for impl_1313 (impl)
macro_rules! Depcrate_query_builder_sql_queryimpl_1313 {
() => {
// Module: crate::query_builder::sql_query
// Provides: {"impl_1313"}
// Dependencies: {}
impl < Query , Value , ST , DB > QueryFragment < DB > for UncheckedBind < Query , Value , ST > where DB : Backend + HasSqlType < ST > + DieselReserveSpecialization , Query : QueryFragment < DB > , Value : ToSql < ST , DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . query . walk_ast (out . reborrow ()) ? ; out . push_bind_param_value_only (& self . value) ? ; Ok (()) } }
};
}
