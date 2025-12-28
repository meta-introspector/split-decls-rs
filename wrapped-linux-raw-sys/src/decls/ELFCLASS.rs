macro_rules! ELFCLASS {
    () => {
        # [cfg (target_pointer_width = "64")] pub const ELFCLASS : u8 = 2 ;
    };
}

ELFCLASS!()