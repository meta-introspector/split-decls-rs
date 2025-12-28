macro_rules! UniversalCompactionStopStyle {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde1" , derive (serde :: Serialize , serde :: Deserialize))] pub enum UniversalCompactionStopStyle { Similar = ffi :: rocksdb_similar_size_compaction_stop_style as isize , Total = ffi :: rocksdb_total_size_compaction_stop_style as isize , }
    };
}

UniversalCompactionStopStyle!()