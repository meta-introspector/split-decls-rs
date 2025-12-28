macro_rules! hi_lo_to_u128 {
    () => {
        # [inline] fn hi_lo_to_u128 (lo : u64 , hi : u64) -> u128 { ((hi as u128) << 64) | (lo as u128) }
    };
}

hi_lo_to_u128!();