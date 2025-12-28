macro_rules! deps {
    () => {
        Transaction!();
    };
}

macro_rules! TransactionState {
    () => {
        deps!();
        # [doc = " Transaction state of a database"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [non_exhaustive] # [cfg (feature = "modern_sqlite")] pub enum TransactionState { # [doc = " Equivalent to `SQLITE_TXN_NONE`"] None , # [doc = " Equivalent to `SQLITE_TXN_READ`"] Read , # [doc = " Equivalent to `SQLITE_TXN_WRITE`"] Write , }
    };
}

TransactionState!();