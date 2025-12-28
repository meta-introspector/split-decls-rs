macro_rules! N_MAX {
    () => {
        # [doc = " scrypt `n * 128 * r` must be less than [i32::MAX]."] pub const N_MAX : u32 = (i32 :: MAX as u32) / 128 ;
    };
}

N_MAX!();