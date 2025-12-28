macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! macro_264 {
    () => {
        deps!();
        fmt_impl ! (LowerHex , Bytes) ;
    };
}

macro_264!();