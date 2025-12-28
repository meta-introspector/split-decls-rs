macro_rules! deps {
    () => {
        Configuration!();
    };
}

macro_rules! standard {
    () => {
        deps!();
        # [doc = " The default config for bincode 2.0. By default this will be:"] # [doc = " - Little endian"] # [doc = " - Variable int encoding"] pub const fn standard () -> Configuration { generate () }
    };
}

standard!()