macro_rules! deps {
    () => {
        Import!();
    };
}

macro_rules! IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT {
    () => {
        deps!();
        # [doc = " Delay Load Import Descriptors"] pub const IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT : usize = 13 ;
    };
}

IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT!()