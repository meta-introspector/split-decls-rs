macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! BufEntry {
    () => {
        deps!();
        struct BufEntry { token : Token , size : isize , }
    };
}

BufEntry!()