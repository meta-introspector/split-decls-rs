macro_rules! HYPER_IO_PENDING {
    () => {
        # [doc = " Sentinel value to return from a read or write callback that the operation"] # [doc = " is pending."] pub const HYPER_IO_PENDING : size_t = 0xFFFFFFFF ;
    };
}

HYPER_IO_PENDING!()