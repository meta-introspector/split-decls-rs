macro_rules! deps {
    () => {
        Import!();
    };
}

macro_rules! IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT {
    () => {
        deps!();
        # [doc = " Bound Import Directory in headers"] pub const IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT : usize = 11 ;
    };
}

IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT!()