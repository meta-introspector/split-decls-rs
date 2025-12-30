// Generated macro for impl_1944 (impl)
macro_rules! Depcrate_r2d2impl_1944 {
() => {
// Module: crate::r2d2
// Provides: {"impl_1944"}
// Dependencies: {}
impl < M , T > TransactionManager < PooledConnection < M > > for PoolTransactionManager < T > where M : ManageConnection , M :: Connection : Connection < TransactionManager = T > + R2D2Connection , T : TransactionManager < M :: Connection > , { type TransactionStateData = T :: TransactionStateData ; fn begin_transaction (conn : & mut PooledConnection < M >) -> QueryResult < () > { T :: begin_transaction (& mut * * conn) } fn rollback_transaction (conn : & mut PooledConnection < M >) -> QueryResult < () > { T :: rollback_transaction (& mut * * conn) } fn commit_transaction (conn : & mut PooledConnection < M >) -> QueryResult < () > { T :: commit_transaction (& mut * * conn) } fn transaction_manager_status_mut (conn : & mut PooledConnection < M > ,) -> & mut TransactionManagerStatus { T :: transaction_manager_status_mut (& mut * * conn) } }
};
}
