// Generated macro for ESTIMATE_PENDING_COMPACTION_BYTES (const)
macro_rules! Depcrate_propertiesESTIMATE_PENDING_COMPACTION_BYTES {
() => {
// Module: crate::properties
// Provides: {"ESTIMATE_PENDING_COMPACTION_BYTES"}
// Dependencies: {}
# [doc = " \"rocksdb.estimate-pending-compaction-bytes\" - returns estimated total"] # [doc = " number of bytes compaction needs to rewrite to get all levels down"] # [doc = " to under target size. Not valid for other compactions than level-"] # [doc = " based."] pub const ESTIMATE_PENDING_COMPACTION_BYTES : & PropName = property ! ("estimate-pending-compaction-bytes") ;
};
}
