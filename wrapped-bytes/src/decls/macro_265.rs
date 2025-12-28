macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! macro_265 {
    () => {
        deps!();
        fmt_impl ! (LowerHex , BytesMut) ;
    };
}

macro_265!()