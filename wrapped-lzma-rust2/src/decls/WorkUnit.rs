macro_rules! WorkUnit {
    () => {
        # [doc = " A work unit for a worker thread."] # [doc = " Contains the sequence number and the raw compressed bytes for a series of chunks."] type WorkUnit = (u64 , Vec < u8 >) ;
    };
}

WorkUnit!()