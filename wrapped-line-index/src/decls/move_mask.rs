macro_rules! move_mask {
    () => {
        # [target_feature (enable = "neon")] # [cfg (all (target_arch = "aarch64" , target_endian = "little"))] # [inline] # [allow (unsafe_op_in_unsafe_fn)] unsafe fn move_mask (v : std :: arch :: aarch64 :: uint8x16_t) -> u64 { use std :: arch :: aarch64 :: * ; let nibble_mask = vshrn_n_u16 (vreinterpretq_u16_u8 (v) , 4) ; vget_lane_u64 (vreinterpret_u64_u8 (nibble_mask) , 0) }
    };
}

move_mask!();