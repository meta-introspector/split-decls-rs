// Generated macro for shuffle_imm_as_le_lane_idx (function)
macro_rules! Depcrate_machinst_isleshuffle_imm_as_le_lane_idx {
() => {
// Module: crate::machinst::isle
// Provides: {"shuffle_imm_as_le_lane_idx"}
// Dependencies: {}
# [doc = " Returns the `size`-byte lane referred to by the shuffle immediate specified"] # [doc = " in `bytes`."] # [doc = ""] # [doc = " This helper is used by `shuffleNN_from_imm` above and is used to interpret a"] # [doc = " byte-based shuffle as a higher-level shuffle of bigger lanes. This will see"] # [doc = " if the `bytes` specified, which must have `size` length, specifies a lane in"] # [doc = " vectors aligned to a `size`-byte boundary."] # [doc = ""] # [doc = " Returns `None` if `bytes` doesn't specify a `size`-byte lane aligned"] # [doc = " appropriately, or returns `Some(n)` where `n` is the index of the lane being"] # [doc = " shuffled."] pub fn shuffle_imm_as_le_lane_idx (size : u8 , bytes : & [u8]) -> Option < u8 > { assert_eq ! (bytes . len () , usize :: from (size)) ; if bytes [0] % size != 0 { return None ; } for i in 0 .. size - 1 { let idx = usize :: from (i) ; if bytes [idx] + 1 != bytes [idx + 1] { return None ; } } Some (bytes [0] / size) }
};
}
