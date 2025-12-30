// Generated macro for NUM_LIVE_VERSIONS (const)
macro_rules! Depcrate_propertiesNUM_LIVE_VERSIONS {
() => {
// Module: crate::properties
// Provides: {"NUM_LIVE_VERSIONS"}
// Dependencies: {}
# [doc = " \"rocksdb.num-live-versions\" - returns number of live versions. `Version`"] # [doc = " is an internal data structure. See version_set.h for details. More"] # [doc = " live versions often mean more SST files are held from being deleted,"] # [doc = " by iterators or unfinished compactions."] pub const NUM_LIVE_VERSIONS : & PropName = property ! ("num-live-versions") ;
};
}
