macro_rules! deps {
    () => {
        RawTable!();
    };
}

macro_rules! RawTableInner {
    () => {
        deps!();
        # [doc = " Non-generic part of `RawTable` which allows functions to be instantiated only once regardless"] # [doc = " of how many different key-value types are used."] struct RawTableInner { bucket_mask : usize , ctrl : NonNull < u8 > , growth_left : usize , items : usize , }
    };
}

RawTableInner!();