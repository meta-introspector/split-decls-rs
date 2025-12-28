macro_rules! deps {
    () => {
        ByteSlice!();
    };
}

macro_rules! private {
    () => {
        deps!();
        # [doc = " Ensure that callers cannot implement `ByteSlice` by making an"] # [doc = " umplementable trait its super trait."] mod private { pub trait Sealed { } }
    };
}

private!()