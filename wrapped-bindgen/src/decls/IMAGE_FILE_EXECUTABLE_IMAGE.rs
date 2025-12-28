macro_rules! deps {
    () => {
        IMAGE_FILE_CHARACTERISTICS!();
    };
}

macro_rules! IMAGE_FILE_EXECUTABLE_IMAGE {
    () => {
        deps!();
        pub const IMAGE_FILE_EXECUTABLE_IMAGE : IMAGE_FILE_CHARACTERISTICS = 2u16 ;
    };
}

IMAGE_FILE_EXECUTABLE_IMAGE!();