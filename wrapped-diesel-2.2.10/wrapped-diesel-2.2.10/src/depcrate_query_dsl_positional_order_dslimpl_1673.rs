// Generated macro for impl_1673 (impl)
macro_rules! Depcrate_query_dsl_positional_order_dslimpl_1673 {
() => {
// Module: crate::query_dsl::positional_order_dsl
// Provides: {"impl_1673"}
// Dependencies: {}
impl < DB : Backend > QueryFragment < DB > for OrderColumn { fn walk_ast < 'b > (& 'b self , mut pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { pass . push_sql (& self . 0 . to_string ()) ; Ok (()) } }
};
}
