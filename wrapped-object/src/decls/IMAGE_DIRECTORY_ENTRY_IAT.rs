macro_rules! deps {
    () => {
        Import!();
        Table!();
    };
}

macro_rules! IMAGE_DIRECTORY_ENTRY_IAT {
    () => {
        deps!();
        # [doc = " Import Address Table"] pub const IMAGE_DIRECTORY_ENTRY_IAT : usize = 12 ;
    };
}

IMAGE_DIRECTORY_ENTRY_IAT!();