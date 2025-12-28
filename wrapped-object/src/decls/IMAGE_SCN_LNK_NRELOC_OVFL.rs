macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_SCN_LNK_NRELOC_OVFL {
    () => {
        deps!();
        # [doc = " Section contains extended relocations."] pub const IMAGE_SCN_LNK_NRELOC_OVFL : u32 = 0x0100_0000 ;
    };
}

IMAGE_SCN_LNK_NRELOC_OVFL!()