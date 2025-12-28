macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! IMAGE_FILE_EXECUTABLE_IMAGE {
    () => {
        deps!();
        # [doc = " File is executable  (i.e. no unresolved external references)."] pub const IMAGE_FILE_EXECUTABLE_IMAGE : u16 = 0x0002 ;
    };
}

IMAGE_FILE_EXECUTABLE_IMAGE!();