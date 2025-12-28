macro_rules! deps {
    () => {
        FORMAT_MESSAGE_OPTIONS!();
    };
}

macro_rules! FORMAT_MESSAGE_FROM_HMODULE {
    () => {
        deps!();
        pub const FORMAT_MESSAGE_FROM_HMODULE : FORMAT_MESSAGE_OPTIONS = 2048u32 ;
    };
}

FORMAT_MESSAGE_FROM_HMODULE!();