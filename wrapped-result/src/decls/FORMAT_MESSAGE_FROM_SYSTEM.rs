macro_rules! deps {
    () => {
        FORMAT_MESSAGE_OPTIONS!();
    };
}

macro_rules! FORMAT_MESSAGE_FROM_SYSTEM {
    () => {
        deps!();
        pub const FORMAT_MESSAGE_FROM_SYSTEM : FORMAT_MESSAGE_OPTIONS = 4096u32 ;
    };
}

FORMAT_MESSAGE_FROM_SYSTEM!();