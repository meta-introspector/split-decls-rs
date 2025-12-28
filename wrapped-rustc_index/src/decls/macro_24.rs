macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! macro_24 {
    () => {
        deps!();
        # [cfg (target_pointer_width = "64")] crate :: static_assert_size ! (Chunk , 16) ;
    };
}

macro_24!();