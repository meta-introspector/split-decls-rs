macro_rules! deps {
    () => {
        Kind!();
        Error!();
    };
}

macro_rules! compute_stream_hash {
    () => {
        deps!();
        # [doc = " A function to compute a hash of kind `hash_kind` for an object of `object_kind` and its data read from `stream`"] # [doc = " which has to yield exactly `stream_len` bytes."] # [doc = " Use `progress` to learn about progress in bytes processed and `should_interrupt` to be able to abort the operation"] # [doc = " if set to `true`."] # [doc (alias = "hash_file" , alias = "git2")] pub fn compute_stream_hash (hash_kind : gix_hash :: Kind , object_kind : Kind , stream : & mut dyn std :: io :: Read , stream_len : u64 , progress : & mut dyn gix_features :: progress :: Progress , should_interrupt : & std :: sync :: atomic :: AtomicBool ,) -> Result < gix_hash :: ObjectId , gix_hash :: io :: Error > { let hasher = object_hasher (hash_kind , object_kind , stream_len) ; gix_hash :: bytes_with_hasher (stream , stream_len , hasher , progress , should_interrupt) }
    };
}

compute_stream_hash!()