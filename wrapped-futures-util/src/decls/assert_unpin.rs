macro_rules! assert_unpin {
    () => {
        # [doc (hidden)] # [inline (always)] pub fn assert_unpin < T : Unpin > (_ : & T) { }
    };
}

assert_unpin!();