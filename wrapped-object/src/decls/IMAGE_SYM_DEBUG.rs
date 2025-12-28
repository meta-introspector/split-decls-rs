macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! IMAGE_SYM_DEBUG {
    () => {
        deps!();
        # [doc = " Symbol is a special debug item."] pub const IMAGE_SYM_DEBUG : i32 = - 2 ;
    };
}

IMAGE_SYM_DEBUG!();