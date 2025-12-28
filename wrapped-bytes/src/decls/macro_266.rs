macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! macro_266 {
    () => {
        deps!();
        fmt_impl ! (UpperHex , Bytes) ;
    };
}

macro_266!();