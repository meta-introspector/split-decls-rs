macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! CF_FILE_HISTOGRAM {
    () => {
        deps!();
        # [doc = " \"rocksdb.cf-file-histogram\" - print out how many file reads to every"] # [doc = " level, as well as the histogram of latency of single requests."] pub const CF_FILE_HISTOGRAM : & PropName = property ! ("cf-file-histogram") ;
    };
}

CF_FILE_HISTOGRAM!();