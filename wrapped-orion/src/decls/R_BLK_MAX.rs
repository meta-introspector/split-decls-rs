macro_rules! R_BLK_MAX {
    () => {
        # [doc = " scrypt `r * 256` must be less than [i32::MAX]."] pub const R_BLK_MAX : u32 = (i32 :: MAX as u32) / 256 ;
    };
}

R_BLK_MAX!();