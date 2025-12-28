macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! OPTIONS_STATISTICS {
    () => {
        deps!();
        # [doc = " \"rocksdb.options-statistics\" - returns multi-line string"] # [doc = " of options.statistics"] pub const OPTIONS_STATISTICS : & PropName = property ! ("options-statistics") ;
    };
}

OPTIONS_STATISTICS!()