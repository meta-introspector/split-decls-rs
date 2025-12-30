// Generated macro for BottommostLevelCompaction (enum)
macro_rules! Depcrate_db_optionsBottommostLevelCompaction {
() => {
// Module: crate::db_options
// Provides: {"BottommostLevelCompaction"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde1" , derive (serde :: Serialize , serde :: Deserialize))] # [repr (u8)] pub enum BottommostLevelCompaction { # [doc = " Skip bottommost level compaction"] Skip = 0 , # [doc = " Only compact bottommost level if there is a compaction filter"] # [doc = " This is the default option"] IfHaveCompactionFilter , # [doc = " Always compact bottommost level"] Force , # [doc = " Always compact bottommost level but in bottommost level avoid"] # [doc = " double-compacting files created in the same compaction"] ForceOptimized , }
};
}
