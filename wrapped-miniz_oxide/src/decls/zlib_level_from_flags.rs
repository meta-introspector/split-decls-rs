macro_rules! zlib_level_from_flags {
    () => {
        # [inline] const fn zlib_level_from_flags (flags : u32) -> u8 { use crate :: deflate :: core :: NUM_PROBES ; let num_probes = flags & super :: MAX_PROBES_MASK ; if (flags & TDEFL_GREEDY_PARSING_FLAG != 0) || (flags & TDEFL_RLE_MATCHES != 0) { if num_probes <= 1 { 0 } else { 1 } } else if num_probes >= NUM_PROBES [9] as u32 { 3 } else { 2 } }
    };
}

zlib_level_from_flags!();