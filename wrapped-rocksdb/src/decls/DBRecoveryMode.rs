macro_rules! DBRecoveryMode {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde1" , derive (serde :: Serialize , serde :: Deserialize))] pub enum DBRecoveryMode { TolerateCorruptedTailRecords = ffi :: rocksdb_tolerate_corrupted_tail_records_recovery as isize , AbsoluteConsistency = ffi :: rocksdb_absolute_consistency_recovery as isize , PointInTime = ffi :: rocksdb_point_in_time_recovery as isize , SkipAnyCorruptedRecord = ffi :: rocksdb_skip_any_corrupted_records_recovery as isize , }
    };
}

DBRecoveryMode!()