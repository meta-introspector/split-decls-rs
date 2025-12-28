macro_rules! is_small {
    () => {
        fn is_small < T > () -> bool { mem :: size_of :: < T > () <= mem :: size_of :: < Value > () && mem :: align_of :: < T > () <= mem :: align_of :: < Value > () }
    };
}

is_small!();