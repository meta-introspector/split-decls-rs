macro_rules! BufList {
    () => {
        pub (crate) struct BufList < T > { bufs : VecDeque < T > , }
    };
}

BufList!();