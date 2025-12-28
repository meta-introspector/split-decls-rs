macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! assert_sink {
    () => {
        deps!();
        pub (crate) fn assert_sink < T , E , S > (sink : S) -> S where S : Sink < T , Error = E > , { sink }
    };
}

assert_sink!();