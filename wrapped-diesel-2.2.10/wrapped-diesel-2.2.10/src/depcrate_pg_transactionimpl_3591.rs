// Generated macro for impl_3591 (impl)
macro_rules! Depcrate_pg_transactionimpl_3591 {
() => {
// Module: crate::pg::transaction
// Provides: {"impl_3591"}
// Dependencies: {}
impl QueryFragment < Pg > for IsolationLevel { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { out . push_sql (" ISOLATION LEVEL ") ; match * self { IsolationLevel :: ReadCommitted => out . push_sql ("READ COMMITTED") , IsolationLevel :: RepeatableRead => out . push_sql ("REPEATABLE READ") , IsolationLevel :: Serializable => out . push_sql ("SERIALIZABLE") , } Ok (()) } }
};
}
