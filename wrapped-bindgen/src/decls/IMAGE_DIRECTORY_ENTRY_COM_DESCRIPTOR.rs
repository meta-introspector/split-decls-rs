macro_rules! deps {
    () => {
        IMAGE_DIRECTORY_ENTRY!();
    };
}

macro_rules! IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR {
    () => {
        deps!();
        pub const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR : IMAGE_DIRECTORY_ENTRY = 14u16 ;
    };
}

IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR!();