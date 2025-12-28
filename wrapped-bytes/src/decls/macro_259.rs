macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! macro_259 {
    () => {
        deps!();
        fmt_impl ! (Debug , Bytes) ;
    };
}

macro_259!();