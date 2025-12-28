macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! macro_267 {
    () => {
        deps!();
        fmt_impl ! (UpperHex , BytesMut) ;
    };
}

macro_267!()