macro_rules! IMAGE_GUARD_RETPOLINE_PRESENT {
    () => {
        # [doc = " Module was built with retpoline support"] pub const IMAGE_GUARD_RETPOLINE_PRESENT : u32 = 0x0010_0000 ;
    };
}

IMAGE_GUARD_RETPOLINE_PRESENT!()