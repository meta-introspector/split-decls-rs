macro_rules! deps {
    () => {
        IteratorType!();
    };
}

macro_rules! peekable {
    () => {
        deps!();
        fn peekable < I : Iterator > (iter : I) -> IteratorType < I > { iter . peekable () }
    };
}

peekable!();