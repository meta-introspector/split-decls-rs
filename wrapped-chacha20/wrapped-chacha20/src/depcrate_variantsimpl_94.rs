// Generated macro for impl_94 (impl)
macro_rules! Depcrate_variantsimpl_94 {
() => {
// Module: crate::variants
// Provides: {"impl_94"}
// Dependencies: {}
# [cfg (any (feature = "legacy" , feature = "rng"))] impl Variant for Legacy { type Counter = u64 ; # [inline (always)] fn get_block_pos (row : & [u32]) -> u64 { (u64 :: from (row [1]) << 32) | u64 :: from (row [0]) } # [inline (always)] fn set_block_pos (row : & mut [u32] , pos : u64) { row [0] = (pos & 0xFFFF_FFFF) . try_into () . unwrap () ; row [1] = (pos >> 32) . try_into () . unwrap () ; } # [inline (always)] fn remaining_blocks (block_pos : u64) -> Option < usize > { let remaining = u64 :: MAX - block_pos ; remaining . try_into () . ok () } }
};
}
