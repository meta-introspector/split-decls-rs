macro_rules! BitBuffer {
    () => {
        # [cfg (not (target_pointer_width = "64"))] type BitBuffer = u32 ;
    };
}

BitBuffer!();