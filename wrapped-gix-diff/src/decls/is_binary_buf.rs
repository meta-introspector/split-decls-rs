macro_rules! is_binary_buf {
    () => {
        fn is_binary_buf (buf : & [u8]) -> bool { let buf = & buf [.. buf . len () . min (8000)] ; buf . contains (& 0) }
    };
}

is_binary_buf!()