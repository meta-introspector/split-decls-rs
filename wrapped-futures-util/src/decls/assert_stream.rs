macro_rules! assert_stream {
    () => {
        pub (crate) fn assert_stream < T , S > (stream : S) -> S where S : Stream < Item = T > , { stream }
    };
}

assert_stream!();