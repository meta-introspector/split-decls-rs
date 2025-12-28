macro_rules! deps {
    () => {
        IMAGE_FILE_CHARACTERISTICS!();
    };
}

macro_rules! IMAGE_FILE_32BIT_MACHINE {
    () => {
        deps!();
        pub const IMAGE_FILE_32BIT_MACHINE : IMAGE_FILE_CHARACTERISTICS = 256u16 ;
    };
}

IMAGE_FILE_32BIT_MACHINE!()