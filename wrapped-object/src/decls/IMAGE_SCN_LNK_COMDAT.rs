macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_SCN_LNK_COMDAT {
    () => {
        deps!();
        # [doc = " Section contents comdat."] pub const IMAGE_SCN_LNK_COMDAT : u32 = 0x0000_1000 ;
    };
}

IMAGE_SCN_LNK_COMDAT!();