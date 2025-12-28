macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! IMAGE_FILE_BYTES_REVERSED_HI {
    () => {
        deps!();
        # [doc = " Bytes of machine word are reversed."] pub const IMAGE_FILE_BYTES_REVERSED_HI : u16 = 0x8000 ;
    };
}

IMAGE_FILE_BYTES_REVERSED_HI!();