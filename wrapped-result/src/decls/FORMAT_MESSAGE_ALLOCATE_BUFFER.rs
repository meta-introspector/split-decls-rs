macro_rules! deps {
    () => {
        FORMAT_MESSAGE_OPTIONS!();
    };
}

macro_rules! FORMAT_MESSAGE_ALLOCATE_BUFFER {
    () => {
        deps!();
        pub const FORMAT_MESSAGE_ALLOCATE_BUFFER : FORMAT_MESSAGE_OPTIONS = 256u32 ;
    };
}

FORMAT_MESSAGE_ALLOCATE_BUFFER!();