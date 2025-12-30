// Generated macro for impl_3541 (impl)
macro_rules! Depcrate_pg_query_builder_query_fragment_implsimpl_3541 {
() => {
// Module: crate::pg::query_builder::query_fragment_impls
// Provides: {"impl_3541"}
// Dependencies: {}
impl < ST , I > QueryFragment < Pg , PgStyleArrayComparison > for Many < ST , I > where ST : SingleValue , Vec < I > : ToSql < Array < ST > , Pg > , Pg : HasSqlType < ST > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { out . push_bind_param :: < Array < ST > , Vec < I > > (& self . values) } }
};
}
