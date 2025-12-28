macro_rules! deps {
    () => {
        IMAGE_OPTIONAL_HEADER_MAGIC!();
    };
}

macro_rules! IMAGE_NT_OPTIONAL_HDR32_MAGIC {
    () => {
        deps!();
        pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC : IMAGE_OPTIONAL_HEADER_MAGIC = 267u16 ;
    };
}

IMAGE_NT_OPTIONAL_HDR32_MAGIC!()