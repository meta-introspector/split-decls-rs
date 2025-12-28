macro_rules! DEFAULT_BUFFER_CAPACITY {
    () => {
        # [doc = " The default buffer capacity that we use for the stream buffer."] const DEFAULT_BUFFER_CAPACITY : usize = 64 * (1 << 10) ;
    };
}

DEFAULT_BUFFER_CAPACITY!();