macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! CFSTATS_NO_FILE_HISTOGRAM {
    () => {
        deps!();
        # [doc = " \"rocksdb.cfstats-no-file-histogram\" - returns a multi-line string with"] # [doc = " general column family stats per-level over db's lifetime (\"`L<n>`\"),"] # [doc = " aggregated over db's lifetime (\"Sum\"), and aggregated over the"] # [doc = " interval since the last retrieval (\"Int\")."] # [doc = " It could also be used to return the stats in the format of the map."] # [doc = " In this case there will be a pair of string to array of double for"] # [doc = " each level as well as for \"Sum\". \"Int\" stats will not be affected"] # [doc = " when this form of stats are retrieved."] pub const CFSTATS_NO_FILE_HISTOGRAM : & PropName = property ! ("cfstats-no-file-histogram") ;
    };
}

CFSTATS_NO_FILE_HISTOGRAM!()