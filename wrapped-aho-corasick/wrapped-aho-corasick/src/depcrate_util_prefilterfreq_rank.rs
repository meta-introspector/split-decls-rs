// Generated macro for freq_rank (function)
macro_rules! Depcrate_util_prefilterfreq_rank {
() => {
// Module: crate::util::prefilter
// Provides: {"freq_rank"}
// Dependencies: {}
# [doc = " Return the frequency rank of the given byte. The higher the rank, the more"] # [doc = " common the byte (heuristically speaking)."] fn freq_rank (b : u8) -> u8 { use crate :: util :: byte_frequencies :: BYTE_FREQUENCIES ; BYTE_FREQUENCIES [b as usize] }
};
}
