// Generated macro for impl_1941 (impl)
macro_rules! Depcrate_r2d2impl_1941 {
() => {
// Module: crate::r2d2
// Provides: {"impl_1941"}
// Dependencies: {}
impl < M > Connection for PooledConnection < M > where M : ManageConnection , M :: Connection : Connection + R2D2Connection + Send + 'static , { type Backend = < M :: Connection as Connection > :: Backend ; type TransactionManager = PoolTransactionManager < < M :: Connection as Connection > :: TransactionManager > ; fn establish (_ : & str) -> ConnectionResult < Self > { Err (ConnectionError :: BadConnection (String :: from ("Cannot directly establish a pooled connection" ,))) } fn begin_test_transaction (& mut self) -> QueryResult < () > { (* * self) . begin_test_transaction () } fn execute_returning_count < T > (& mut self , source : & T) -> QueryResult < usize > where T : QueryFragment < Self :: Backend > + QueryId , { (* * self) . execute_returning_count (source) } fn transaction_state (& mut self ,) -> & mut < Self :: TransactionManager as TransactionManager < Self > > :: TransactionStateData { (* * self) . transaction_state () } fn instrumentation (& mut self) -> & mut dyn crate :: connection :: Instrumentation { (* * self) . instrumentation () } fn set_instrumentation (& mut self , instrumentation : impl crate :: connection :: Instrumentation) { (* * self) . set_instrumentation (instrumentation) } }
};
}
