macro_rules! deps {
    () => {
        LittleEndian!();
        Fixint!();
        NoLimit!();
        Configuration!();
    };
}

macro_rules! legacy {
    () => {
        deps!();
        # [doc = " Creates the \"legacy\" default config. This is the default config that was present in bincode 1.0"] # [doc = " - Little endian"] # [doc = " - Fixed int length encoding"] pub const fn legacy () -> Configuration < LittleEndian , Fixint , NoLimit > { generate () }
    };
}

legacy!();