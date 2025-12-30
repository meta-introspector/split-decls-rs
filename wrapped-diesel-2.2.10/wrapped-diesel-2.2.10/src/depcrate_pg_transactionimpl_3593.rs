// Generated macro for impl_3593 (impl)
macro_rules! Depcrate_pg_transactionimpl_3593 {
() => {
// Module: crate::pg::transaction
// Provides: {"impl_3593"}
// Dependencies: {}
impl QueryFragment < Pg > for ReadMode { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { match * self { ReadMode :: ReadOnly => out . push_sql (" READ ONLY") , ReadMode :: ReadWrite => out . push_sql (" READ WRITE") , } Ok (()) } }
};
}
