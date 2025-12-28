macro_rules! deps {
    () => {
        OptimisticTransactionOptions!();
    };
}

macro_rules! impl_443 {
    () => {
        deps!();
        impl OptimisticTransactionOptions { pub fn new () -> OptimisticTransactionOptions { OptimisticTransactionOptions :: default () } # [doc = " Specifies use snapshot or not."] # [doc = ""] # [doc = " Default: false."] # [doc = ""] # [doc = " If a transaction has a snapshot set, the transaction will ensure that"] # [doc = " any keys successfully written(or fetched via `get_for_update`) have not"] # [doc = " been modified outside the transaction since the time the snapshot was"] # [doc = " set."] # [doc = " If a snapshot has not been set, the transaction guarantees that keys have"] # [doc = " not been modified since the time each key was first written (or fetched via"] # [doc = " `get_for_update`)."] # [doc = ""] # [doc = " Using snapshot will provide stricter isolation guarantees at the"] # [doc = " expense of potentially more transaction failures due to conflicts with"] # [doc = " other writes."] # [doc = ""] # [doc = " Calling `set_snapshot` will not affect the version of Data returned by `get`"] # [doc = " methods."] pub fn set_snapshot (& mut self , snapshot : bool) { unsafe { ffi :: rocksdb_optimistictransaction_options_set_set_snapshot (self . inner , u8 :: from (snapshot) ,) ; } } }
    };
}

impl_443!()