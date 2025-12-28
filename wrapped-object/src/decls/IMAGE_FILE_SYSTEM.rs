macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! IMAGE_FILE_SYSTEM {
    () => {
        deps!();
        # [doc = " System File."] pub const IMAGE_FILE_SYSTEM : u16 = 0x1000 ;
    };
}

IMAGE_FILE_SYSTEM!()