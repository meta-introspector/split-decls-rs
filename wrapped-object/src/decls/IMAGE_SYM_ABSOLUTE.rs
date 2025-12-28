macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! IMAGE_SYM_ABSOLUTE {
    () => {
        deps!();
        # [doc = " Symbol is an absolute value."] pub const IMAGE_SYM_ABSOLUTE : i32 = - 1 ;
    };
}

IMAGE_SYM_ABSOLUTE!();