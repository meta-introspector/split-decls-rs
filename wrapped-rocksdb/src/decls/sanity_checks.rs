macro_rules! sanity_checks {
    () => {
        # [test] fn sanity_checks () { let want = "rocksdb.async.read.bytes" ; assert_eq ! (want , Histogram :: AsyncReadBytes . name ()) ; let want = "rocksdb.block.cache.index.miss" ; assert_eq ! (want , Ticker :: BlockCacheIndexMiss . to_string ()) ; assert_eq ! (Ticker :: iter () . count () , 211) ; assert_eq ! (Histogram :: iter () . count () , 62) ; }
    };
}

sanity_checks!();