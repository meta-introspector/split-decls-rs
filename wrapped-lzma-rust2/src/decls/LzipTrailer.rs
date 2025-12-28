macro_rules! LzipTrailer {
    () => {
        # [derive (Debug , Clone)] pub (crate) struct LzipTrailer { crc32 : u32 , data_size : u64 , member_size : u64 , }
    };
}

LzipTrailer!();