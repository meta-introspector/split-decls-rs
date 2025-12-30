// Generated macro for MemtableFactory (enum)
macro_rules! Depcrate_db_optionsMemtableFactory {
() => {
// Module: crate::db_options
// Provides: {"MemtableFactory"}
// Dependencies: {}
# [doc = " Defines the underlying memtable implementation."] # [doc = " See official [wiki](https://github.com/facebook/rocksdb/wiki/MemTable) for more information."] pub enum MemtableFactory { Vector , HashSkipList { bucket_count : usize , height : i32 , branching_factor : i32 , } , HashLinkList { bucket_count : usize , } , }
};
}
