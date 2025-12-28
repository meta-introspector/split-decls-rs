macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! IMAGE_FILE_UP_SYSTEM_ONLY {
    () => {
        deps!();
        # [doc = " File should only be run on a UP machine"] pub const IMAGE_FILE_UP_SYSTEM_ONLY : u16 = 0x4000 ;
    };
}

IMAGE_FILE_UP_SYSTEM_ONLY!()