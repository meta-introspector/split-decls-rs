macro_rules! IMAGE_SCN_NO_DEFER_SPEC_EXC {
    () => {
        # [doc = " Reset speculative exceptions handling bits in the TLB entries for this section."] pub const IMAGE_SCN_NO_DEFER_SPEC_EXC : u32 = 0x0000_4000 ;
    };
}

IMAGE_SCN_NO_DEFER_SPEC_EXC!()