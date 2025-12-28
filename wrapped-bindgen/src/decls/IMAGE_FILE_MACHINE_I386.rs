macro_rules! deps {
    () => {
        IMAGE_FILE_MACHINE!();
    };
}

macro_rules! IMAGE_FILE_MACHINE_I386 {
    () => {
        deps!();
        pub const IMAGE_FILE_MACHINE_I386 : IMAGE_FILE_MACHINE = 332u16 ;
    };
}

IMAGE_FILE_MACHINE_I386!()