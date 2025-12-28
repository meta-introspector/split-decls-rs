macro_rules! assert_read {
    () => {
        pub (crate) fn assert_read < R > (reader : R) -> R where R : AsyncRead , { reader }
    };
}

assert_read!();