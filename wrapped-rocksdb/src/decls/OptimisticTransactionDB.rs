macro_rules! deps {
    () => {
        MultiThreaded!();
        OptimisticTransactionDBInner!();
        DBCommon!();
    };
}

macro_rules! OptimisticTransactionDB {
    () => {
        deps!();
        # [cfg (feature = "multi-threaded-cf")] pub type OptimisticTransactionDB < T = crate :: MultiThreaded > = DBCommon < T , OptimisticTransactionDBInner > ;
    };
}

OptimisticTransactionDB!()