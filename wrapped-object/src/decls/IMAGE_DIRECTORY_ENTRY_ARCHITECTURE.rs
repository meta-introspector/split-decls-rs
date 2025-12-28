macro_rules! deps {
    () => {
        Architecture!();
    };
}

macro_rules! IMAGE_DIRECTORY_ENTRY_ARCHITECTURE {
    () => {
        deps!();
        # [doc = " Architecture Specific Data"] pub const IMAGE_DIRECTORY_ENTRY_ARCHITECTURE : usize = 7 ;
    };
}

IMAGE_DIRECTORY_ENTRY_ARCHITECTURE!();