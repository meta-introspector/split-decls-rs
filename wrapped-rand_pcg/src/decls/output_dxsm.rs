macro_rules! output_dxsm {
    () => {
        # [inline (always)] fn output_dxsm (state : u128) -> u64 { let mut hi = (state >> 64) as u64 ; let mut lo = state as u64 ; lo |= 1 ; hi ^= hi >> 32 ; hi = hi . wrapping_mul (MULTIPLIER) ; hi ^= hi >> 48 ; hi = hi . wrapping_mul (lo) ; hi }
    };
}

output_dxsm!()