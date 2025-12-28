macro_rules! cmf_from_flags {
    () => {
        # [inline] const fn cmf_from_flags (flags : u32) -> u8 { if (flags & TDEFL_RLE_MATCHES == 0) && (flags & TDEFL_FORCE_ALL_RAW_BLOCKS == 0) { DEFAULT_CMF } else { MIN_CMF } }
    };
}

cmf_from_flags!()