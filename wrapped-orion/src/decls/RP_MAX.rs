macro_rules! RP_MAX {
    () => {
        # [doc = " scrypt `r * p` must be less than `2^30`."] pub const RP_MAX : u64 = 1 << 30 ;
    };
}

RP_MAX!();