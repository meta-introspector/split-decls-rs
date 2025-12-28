macro_rules! deps {
    () => {
        BitFlags!();
    };
}

macro_rules! ImplementedByBitFlagsMacro {
    () => {
        deps!();
        # [doc = " A marker trait that signals that an implementation of `BitFlags` came from the `bitflags!` macro."] # [doc = ""] # [doc = " There's nothing stopping an end-user from implementing this trait, but we don't guarantee their"] # [doc = " manual implementations won't break between non-breaking releases."] # [doc (hidden)] pub trait ImplementedByBitFlagsMacro { }
    };
}

ImplementedByBitFlagsMacro!();