macro_rules! RP_BLK_MAX {
    () => {
        # [doc = " scrypt `r * 128 * p` must be less than [i32::MAX]."] pub const RP_BLK_MAX : u32 = (i32 :: MAX as u32) / 128 ;
    };
}

RP_BLK_MAX!()