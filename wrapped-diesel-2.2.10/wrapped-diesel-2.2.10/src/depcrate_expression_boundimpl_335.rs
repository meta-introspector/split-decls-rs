// Generated macro for impl_335 (impl)
macro_rules! Depcrate_expression_boundimpl_335 {
() => {
// Module: crate::expression::bound
// Provides: {"impl_335"}
// Dependencies: {}
impl < T , U , DB > QueryFragment < DB > for Bound < T , U > where DB : Backend + HasSqlType < T > , U : ToSql < T , DB > , { fn walk_ast < 'b > (& 'b self , mut pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { pass . push_bind_param (& self . item) ? ; Ok (()) } }
};
}
