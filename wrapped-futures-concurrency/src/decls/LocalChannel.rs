macro_rules! LocalChannel {
    () => {
        pub (crate) struct LocalChannel < T > { queue : VecDeque < T > , waker : Option < Waker > , closed : bool , }
    };
}

LocalChannel!()