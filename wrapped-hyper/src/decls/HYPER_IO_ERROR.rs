macro_rules! HYPER_IO_ERROR {
    () => {
        # [doc = " Sentinel value to return from a read or write callback that the operation"] # [doc = " has errored."] pub const HYPER_IO_ERROR : size_t = 0xFFFFFFFE ;
    };
}

HYPER_IO_ERROR!();