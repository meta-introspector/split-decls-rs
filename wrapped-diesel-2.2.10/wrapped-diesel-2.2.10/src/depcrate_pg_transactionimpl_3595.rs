// Generated macro for impl_3595 (impl)
macro_rules! Depcrate_pg_transactionimpl_3595 {
() => {
// Module: crate::pg::transaction
// Provides: {"impl_3595"}
// Dependencies: {}
impl QueryFragment < Pg > for Deferrable { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { match * self { Deferrable :: Deferrable => out . push_sql (" DEFERRABLE") , Deferrable :: NotDeferrable => out . push_sql (" NOT DEFERRABLE") , } Ok (()) } }
};
}
