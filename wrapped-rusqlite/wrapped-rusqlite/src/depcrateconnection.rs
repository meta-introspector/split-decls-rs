// Generated macro for Connection (struct)
macro_rules! DepcrateConnection {
() => {
// Module: crate
// Provides: {"Connection"}
// Dependencies: {}
# [doc = " A connection to a SQLite database."] pub struct Connection { db : RefCell < InnerConnection > , # [cfg (feature = "cache")] cache : StatementCache , transaction_behavior : TransactionBehavior , }
};
}
