macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! AGGREGATED_TABLE_PROPERTIES {
    () => {
        deps!();
        # [doc = " \"rocksdb.aggregated-table-properties\" - returns a string representation"] # [doc = " of the aggregated table properties of the target column family."] pub const AGGREGATED_TABLE_PROPERTIES : & PropName = property ! ("aggregated-table-properties") ;
    };
}

AGGREGATED_TABLE_PROPERTIES!()