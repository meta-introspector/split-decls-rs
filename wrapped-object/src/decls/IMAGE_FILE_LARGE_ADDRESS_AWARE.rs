macro_rules! IMAGE_FILE_LARGE_ADDRESS_AWARE {
    () => {
        # [doc = " App can handle >2gb addresses"] pub const IMAGE_FILE_LARGE_ADDRESS_AWARE : u16 = 0x0020 ;
    };
}

IMAGE_FILE_LARGE_ADDRESS_AWARE!();