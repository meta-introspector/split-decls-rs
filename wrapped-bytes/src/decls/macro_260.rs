macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! macro_260 {
    () => {
        deps!();
        fmt_impl ! (Debug , BytesMut) ;
    };
}

macro_260!()