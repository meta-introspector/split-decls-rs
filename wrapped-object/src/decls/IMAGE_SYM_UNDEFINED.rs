macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! IMAGE_SYM_UNDEFINED {
    () => {
        deps!();
        # [doc = " Symbol is undefined or is common."] pub const IMAGE_SYM_UNDEFINED : i32 = 0 ;
    };
}

IMAGE_SYM_UNDEFINED!()