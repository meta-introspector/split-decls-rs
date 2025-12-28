macro_rules! deps {
    () => {
        Flags!();
    };
}

macro_rules! macro_71 {
    () => {
        deps!();
        __declare_public_bitflags ! { # [doc = " This is the same `Flags` struct defined in the [crate level example](../index.html#example)."] # [doc = " Note that this struct is just for documentation purposes only, it must not be used outside"] # [doc = " this crate."] pub struct Flags }
    };
}

macro_71!();