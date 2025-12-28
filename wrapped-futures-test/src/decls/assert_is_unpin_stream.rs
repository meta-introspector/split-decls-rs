macro_rules! assert_is_unpin_stream {
    () => {
        # [doc (hidden)] pub fn assert_is_unpin_stream < S : Stream + Unpin > (_ : & mut S) { }
    };
}

assert_is_unpin_stream!();