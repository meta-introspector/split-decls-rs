macro_rules! MemtableFactory {
    () => {
        # [doc = " Defines the underlying memtable implementation."] # [doc = " See official [wiki](https://github.com/facebook/rocksdb/wiki/MemTable) for more information."] pub enum MemtableFactory { Vector , HashSkipList { bucket_count : usize , height : i32 , branching_factor : i32 , } , HashLinkList { bucket_count : usize , } , }
    };
}

MemtableFactory!();