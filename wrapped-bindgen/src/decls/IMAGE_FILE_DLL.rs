macro_rules! deps {
    () => {
        IMAGE_FILE_CHARACTERISTICS!();
    };
}

macro_rules! IMAGE_FILE_DLL {
    () => {
        deps!();
        pub const IMAGE_FILE_DLL : IMAGE_FILE_CHARACTERISTICS = 8192u16 ;
    };
}

IMAGE_FILE_DLL!()