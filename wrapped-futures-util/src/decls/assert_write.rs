macro_rules! assert_write {
    () => {
        pub (crate) fn assert_write < W > (writer : W) -> W where W : AsyncWrite , { writer }
    };
}

assert_write!()