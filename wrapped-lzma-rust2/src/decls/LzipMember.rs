macro_rules! LzipMember {
    () => {
        # [cfg (feature = "std")] # [derive (Debug , Clone)] struct LzipMember { start_pos : u64 , compressed_size : u64 , }
    };
}

LzipMember!()