macro_rules! deps {
    () => {
        FILE_FLAGS_AND_ATTRIBUTES!();
    };
}

macro_rules! FILE_ATTRIBUTE_TEMPORARY {
    () => {
        deps!();
        pub const FILE_ATTRIBUTE_TEMPORARY : FILE_FLAGS_AND_ATTRIBUTES = 256u32 ;
    };
}

FILE_ATTRIBUTE_TEMPORARY!();