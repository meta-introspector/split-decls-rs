// Generated macro for impl_1318 (impl)
macro_rules! Depcrate_query_builder_sql_queryimpl_1318 {
() => {
// Module: crate::query_builder::sql_query
// Provides: {"impl_1318"}
// Dependencies: {}
impl < ST , U , DB > QueryFragment < DB > for RawBind < ST , U > where DB : Backend + HasSqlType < ST > , U : ToSql < ST , DB > , { fn walk_ast < 'b > (& 'b self , mut pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { pass . push_bind_param_value_only (& self . value) } }
};
}
