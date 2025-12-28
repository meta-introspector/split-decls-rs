macro_rules! IndexRecord {
    () => {
        # [derive (Debug , Clone)] struct IndexRecord { unpadded_size : u64 , uncompressed_size : u64 , }
    };
}

IndexRecord!();