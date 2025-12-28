macro_rules! TransactionBehavior {
    () => {
        # [doc = " Options for transaction behavior. See [BEGIN"] # [doc = " TRANSACTION](http://www.sqlite.org/lang_transaction.html) for details."] # [derive (Copy , Clone)] # [non_exhaustive] pub enum TransactionBehavior { # [doc = " DEFERRED means that the transaction does not actually start until the"] # [doc = " database is first accessed."] Deferred , # [doc = " IMMEDIATE cause the database connection to start a new write"] # [doc = " immediately, without waiting for a writes statement."] Immediate , # [doc = " EXCLUSIVE prevents other database connections from reading the database"] # [doc = " while the transaction is underway."] Exclusive , }
    };
}

TransactionBehavior!()