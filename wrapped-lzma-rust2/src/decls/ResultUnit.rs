macro_rules! ResultUnit {
    () => {
        # [doc = " A result unit from a worker thread."] # [doc = " Contains the sequence number and the decompressed data."] type ResultUnit = (u64 , Vec < u8 >) ;
    };
}

ResultUnit!();