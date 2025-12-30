// Generated macro for ReadTier (enum)
macro_rules! Depcrate_db_optionsReadTier {
() => {
// Module: crate::db_options
// Provides: {"ReadTier"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde1" , derive (serde :: Serialize , serde :: Deserialize))] # [repr (i32)] pub enum ReadTier { # [doc = " Reads data in memtable, block cache, OS cache or storage."] All = 0 , # [doc = " Reads data in memtable or block cache."] BlockCache , # [doc = " Reads persisted data. When WAL is disabled, this option will skip data in memtable."] Persisted , # [doc = " Reads data in memtable. Used for memtable only iterators."] Memtable , }
};
}
