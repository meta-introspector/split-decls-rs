macro_rules! deps {
    () => {
        InnerConnection!();
        TransactionBehavior!();
        StatementCache!();
    };
}

macro_rules! Connection {
    () => {
        deps!();
        # [doc = " A connection to a SQLite database."] pub struct Connection { db : RefCell < InnerConnection > , # [cfg (feature = "cache")] cache : StatementCache , transaction_behavior : TransactionBehavior , }
    };
}

Connection!();