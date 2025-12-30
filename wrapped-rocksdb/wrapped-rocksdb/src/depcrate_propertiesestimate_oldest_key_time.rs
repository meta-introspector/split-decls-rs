// Generated macro for ESTIMATE_OLDEST_KEY_TIME (const)
macro_rules! Depcrate_propertiesESTIMATE_OLDEST_KEY_TIME {
() => {
// Module: crate::properties
// Provides: {"ESTIMATE_OLDEST_KEY_TIME"}
// Dependencies: {}
# [doc = " \"rocksdb.estimate-oldest-key-time\" - returns an estimation of"] # [doc = " oldest key timestamp in the DB. Currently only available for"] # [doc = " FIFO compaction with"] # [doc = " compaction_options_fifo.allow_compaction = false."] pub const ESTIMATE_OLDEST_KEY_TIME : & PropName = property ! ("estimate-oldest-key-time") ;
};
}
