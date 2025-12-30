// Generated macro for TransactionBuilder (struct)
macro_rules! Depcrate_pg_transactionTransactionBuilder {
() => {
// Module: crate::pg::transaction
// Provides: {"TransactionBuilder"}
// Dependencies: {}
# [doc = " Used to build a transaction, specifying additional details."] # [doc = ""] # [doc = " This struct is returned by [`.build_transaction`]."] # [doc = " See the documentation for methods on this struct for usage examples."] # [doc = " See [the PostgreSQL documentation for `SET TRANSACTION`][pg-docs]"] # [doc = " for details on the behavior of each option."] # [doc = ""] # [doc = " [`.build_transaction`]: PgConnection::build_transaction()"] # [doc = " [pg-docs]: https://www.postgresql.org/docs/current/static/sql-set-transaction.html"] # [allow (missing_debug_implementations)] # [must_use = "Transaction builder does nothing unless you call `run` on it"] # [cfg (feature = "postgres_backend")] pub struct TransactionBuilder < 'a , C > { connection : & 'a mut C , isolation_level : Option < IsolationLevel > , read_mode : Option < ReadMode > , deferrable : Option < Deferrable > , }
};
}
