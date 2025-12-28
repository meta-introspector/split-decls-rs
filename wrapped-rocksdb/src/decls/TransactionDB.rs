macro_rules! deps {
    () => {
        OptionsMustOutliveDB!();
        SingleThreaded!();
        DefaultThreadMode!();
        Options!();
        ThreadMode!();
    };
}

macro_rules! TransactionDB {
    () => {
        deps!();
        # [doc = " RocksDB TransactionDB."] # [doc = ""] # [doc = " Please read the official [guide](https://github.com/facebook/rocksdb/wiki/Transactions)"] # [doc = " to learn more about RocksDB TransactionDB."] # [doc = ""] # [doc = " The default thread mode for [`TransactionDB`] is [`SingleThreaded`]"] # [doc = " if feature `multi-threaded-cf` is not enabled."] # [doc = ""] # [doc = " ```"] # [doc = " use rocksdb::{DB, Options, TransactionDB, SingleThreaded};"] # [doc = " let tempdir = tempfile::Builder::new()"] # [doc = "     .prefix(\"_path_for_transaction_db\")"] # [doc = "     .tempdir()"] # [doc = "     .expect(\"Failed to create temporary path for the _path_for_transaction_db\");"] # [doc = " let path = tempdir.path();"] # [doc = " {"] # [doc = "     let db: TransactionDB = TransactionDB::open_default(path).unwrap();"] # [doc = "     db.put(b\"my key\", b\"my value\").unwrap();"] # [doc = ""] # [doc = "     // create transaction"] # [doc = "     let txn = db.transaction();"] # [doc = "     txn.put(b\"key2\", b\"value2\");"] # [doc = "     txn.put(b\"key3\", b\"value3\");"] # [doc = "     txn.commit().unwrap();"] # [doc = " }"] # [doc = " let _ = DB::destroy(&Options::default(), path);"] # [doc = " ```"] # [doc = ""] # [doc = " [`SingleThreaded`]: crate::SingleThreaded"] pub struct TransactionDB < T : ThreadMode = DefaultThreadMode > { pub (crate) inner : * mut ffi :: rocksdb_transactiondb_t , cfs : T , path : PathBuf , prepared : Mutex < Vec < * mut rocksdb_transaction_t > > , _outlive : Vec < OptionsMustOutliveDB > , }
    };
}

TransactionDB!()