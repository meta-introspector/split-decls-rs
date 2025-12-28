macro_rules! TransactionOperation {
    () => {
        # [doc = " A transaction operation."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] # [non_exhaustive] # [allow (missing_docs)] pub enum TransactionOperation { Unknown , Begin , Release , Rollback , }
    };
}

TransactionOperation!()