macro_rules! PTR_WIDTH {
    () => {
        # [cfg (target_pointer_width = "32")] const PTR_WIDTH : usize = 32 ;
    };
}

PTR_WIDTH!();