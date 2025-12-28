macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! IMAGE_FILE_BYTES_REVERSED_LO {
    () => {
        deps!();
        # [doc = " Bytes of machine word are reversed."] pub const IMAGE_FILE_BYTES_REVERSED_LO : u16 = 0x0080 ;
    };
}

IMAGE_FILE_BYTES_REVERSED_LO!()