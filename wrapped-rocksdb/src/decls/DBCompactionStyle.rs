macro_rules! DBCompactionStyle {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde1" , derive (serde :: Serialize , serde :: Deserialize))] pub enum DBCompactionStyle { Level = ffi :: rocksdb_level_compaction as isize , Universal = ffi :: rocksdb_universal_compaction as isize , Fifo = ffi :: rocksdb_fifo_compaction as isize , }
    };
}

DBCompactionStyle!();