// Generated macro for impl_3589 (impl)
macro_rules! Depcrate_pg_transactionimpl_3589 {
() => {
// Module: crate::pg::transaction
// Provides: {"impl_3589"}
// Dependencies: {}
impl < C > QueryFragment < Pg > for TransactionBuilder < '_ , C > { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Pg >) -> QueryResult < () > { out . push_sql ("BEGIN TRANSACTION") ; if let Some (ref isolation_level) = self . isolation_level { isolation_level . walk_ast (out . reborrow ()) ? ; } if let Some (ref read_mode) = self . read_mode { read_mode . walk_ast (out . reborrow ()) ? ; } if let Some (ref deferrable) = self . deferrable { deferrable . walk_ast (out . reborrow ()) ? ; } Ok (()) } }
};
}
