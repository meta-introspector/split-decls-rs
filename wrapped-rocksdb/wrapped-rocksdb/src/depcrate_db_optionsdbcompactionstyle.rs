// Generated macro for DBCompactionStyle (enum)
macro_rules! Depcrate_db_optionsDBCompactionStyle {
() => {
// Module: crate::db_options
// Provides: {"DBCompactionStyle"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde1" , derive (serde :: Serialize , serde :: Deserialize))] pub enum DBCompactionStyle { Level = ffi :: rocksdb_level_compaction as isize , Universal = ffi :: rocksdb_universal_compaction as isize , Fifo = ffi :: rocksdb_fifo_compaction as isize , }
};
}
