macro_rules! deps {
    () => {
        IMAGE_FILE_MACHINE!();
    };
}

macro_rules! IMAGE_FILE_MACHINE_AMD64 {
    () => {
        deps!();
        pub const IMAGE_FILE_MACHINE_AMD64 : IMAGE_FILE_MACHINE = 34404u16 ;
    };
}

IMAGE_FILE_MACHINE_AMD64!();