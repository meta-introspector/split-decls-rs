macro_rules! CountingWriter {
    () => {
        # [cfg (feature = "encoder")] struct CountingWriter < W > { inner : W , bytes_written : u64 , }
    };
}

CountingWriter!();