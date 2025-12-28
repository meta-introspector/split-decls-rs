macro_rules! deps {
    () => {
        Section!();
    };
}

macro_rules! IMAGE_SCN_LNK_INFO {
    () => {
        deps!();
        # [doc = " Section contains comments or some other type of information."] pub const IMAGE_SCN_LNK_INFO : u32 = 0x0000_0200 ;
    };
}

IMAGE_SCN_LNK_INFO!()