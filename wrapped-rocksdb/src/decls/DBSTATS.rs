macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! DBSTATS {
    () => {
        deps!();
        # [doc = " \"rocksdb.dbstats\" - returns a multi-line string with general database"] # [doc = " stats, both cumulative (over the db's lifetime) and interval (since"] # [doc = " the last retrieval of kDBStats)."] pub const DBSTATS : & PropName = property ! ("dbstats") ;
    };
}

DBSTATS!()