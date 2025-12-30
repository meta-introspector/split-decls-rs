// Generated macro for sanity_checks (function)
macro_rules! Depcrate_statisticssanity_checks {
() => {
// Module: crate::statistics
// Provides: {"sanity_checks"}
// Dependencies: {}
# [test] fn sanity_checks () { let want = "rocksdb.async.read.bytes" ; assert_eq ! (want , Histogram :: AsyncReadBytes . name ()) ; let want = "rocksdb.block.cache.index.miss" ; assert_eq ! (want , Ticker :: BlockCacheIndexMiss . to_string ()) ; assert_eq ! (Ticker :: iter () . count () , 211) ; assert_eq ! (Histogram :: iter () . count () , 62) ; }
};
}
