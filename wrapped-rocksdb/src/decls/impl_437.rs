macro_rules! deps {
    () => {
        TransactionDBOptions!();
        DB!();
        TransactionOptions!();
    };
}

macro_rules! impl_437 {
    () => {
        deps!();
        impl TransactionDBOptions { pub fn new () -> TransactionDBOptions { TransactionDBOptions :: default () } # [doc = " Specifies the wait timeout in milliseconds when writing a key"] # [doc = " outside a transaction (i.e. by calling `TransactionDB::put` directly)."] # [doc = ""] # [doc = " If 0, no waiting is done if a lock cannot instantly be acquired."] # [doc = " If negative, there is no timeout and will block indefinitely when acquiring"] # [doc = " a lock."] # [doc = ""] # [doc = " Not using a timeout can lead to deadlocks.  Currently, there"] # [doc = " is no deadlock-detection to recover from a deadlock.  While DB writes"] # [doc = " cannot deadlock with other DB writes, they can deadlock with a transaction."] # [doc = " A negative timeout should only be used if all transactions have a small"] # [doc = " expiration set."] # [doc = ""] # [doc = " Default: 1000(1s)."] pub fn set_default_lock_timeout (& mut self , default_lock_timeout : i64) { unsafe { ffi :: rocksdb_transactiondb_options_set_default_lock_timeout (self . inner , default_lock_timeout ,) ; } } # [doc = " Specifies the default wait timeout in milliseconds when a transaction"] # [doc = " attempts to lock a key if not specified in `TransactionOptions`."] # [doc = ""] # [doc = " If 0, no waiting is done if a lock cannot instantly be acquired."] # [doc = " If negative, there is no timeout.  Not using a timeout is not recommended"] # [doc = " as it can lead to deadlocks.  Currently, there is no deadlock-detection to"] # [doc = " recover from a deadlock."] # [doc = ""] # [doc = " Default: 1000(1s)."] pub fn set_txn_lock_timeout (& mut self , txn_lock_timeout : i64) { unsafe { ffi :: rocksdb_transactiondb_options_set_transaction_lock_timeout (self . inner , txn_lock_timeout ,) ; } } # [doc = " Specifies the maximum number of keys that can be locked at the same time"] # [doc = " per column family."] # [doc = ""] # [doc = " If the number of locked keys is greater than `max_num_locks`, transaction"] # [doc = " `writes` (or `get_for_update`) will return an error."] # [doc = " If this value is not positive, no limit will be enforced."] # [doc = ""] # [doc = " Default: -1."] pub fn set_max_num_locks (& mut self , max_num_locks : i64) { unsafe { ffi :: rocksdb_transactiondb_options_set_max_num_locks (self . inner , max_num_locks) ; } } # [doc = " Specifies lock table stripes count."] # [doc = ""] # [doc = " Increasing this value will increase the concurrency by dividing the lock"] # [doc = " table (per column family) into more sub-tables, each with their own"] # [doc = " separate mutex."] # [doc = ""] # [doc = " Default: 16."] pub fn set_num_stripes (& mut self , num_stripes : usize) { unsafe { ffi :: rocksdb_transactiondb_options_set_num_stripes (self . inner , num_stripes) ; } } }
    };
}

impl_437!()