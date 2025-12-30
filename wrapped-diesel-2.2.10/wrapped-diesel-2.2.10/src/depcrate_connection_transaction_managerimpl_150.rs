// Generated macro for impl_150 (impl)
macro_rules! Depcrate_connection_transaction_managerimpl_150 {
() => {
// Module: crate::connection::transaction_manager
// Provides: {"impl_150"}
// Dependencies: {}
impl AnsiTransactionManager { fn get_transaction_state < Conn > (conn : & mut Conn ,) -> QueryResult < & mut ValidTransactionManagerStatus > where Conn : Connection < TransactionManager = Self > , { conn . transaction_state () . status . transaction_state () } # [doc = " Begin a transaction with custom SQL"] # [doc = ""] # [doc = " This is used by connections to implement more complex transaction APIs"] # [doc = " to set things such as isolation levels."] # [doc = " Returns an error if already inside of a transaction."] pub fn begin_transaction_sql < Conn > (conn : & mut Conn , sql : & str) -> QueryResult < () > where Conn : Connection < TransactionManager = Self > , { let state = Self :: get_transaction_state (conn) ? ; if let Some (_depth) = state . transaction_depth () { return Err (Error :: AlreadyInTransaction) ; } let instrumentation_depth = NonZeroU32 :: new (1) ; conn . instrumentation () . on_connection_event (super :: instrumentation :: InstrumentationEvent :: BeginTransaction { depth : instrumentation_depth . expect ("We know that 1 is not zero") , } ,) ; conn . batch_execute (sql) ? ; Self :: get_transaction_state (conn) ? . change_transaction_depth (TransactionDepthChange :: IncreaseDepth) ? ; Ok (()) } }
};
}
