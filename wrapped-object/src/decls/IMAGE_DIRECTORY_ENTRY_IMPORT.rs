macro_rules! deps {
    () => {
        Import!();
    };
}

macro_rules! IMAGE_DIRECTORY_ENTRY_IMPORT {
    () => {
        deps!();
        # [doc = " Import Directory"] pub const IMAGE_DIRECTORY_ENTRY_IMPORT : usize = 1 ;
    };
}

IMAGE_DIRECTORY_ENTRY_IMPORT!();