macro_rules! deps {
    () => {
        FORMAT_MESSAGE_OPTIONS!();
    };
}

macro_rules! FORMAT_MESSAGE_IGNORE_INSERTS {
    () => {
        deps!();
        pub const FORMAT_MESSAGE_IGNORE_INSERTS : FORMAT_MESSAGE_OPTIONS = 512u32 ;
    };
}

FORMAT_MESSAGE_IGNORE_INSERTS!()