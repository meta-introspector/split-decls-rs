macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! BufEntry {
    () => {
        deps!();
        # [derive (Clone)] struct BufEntry { token : Token , size : isize , }
    };
}

BufEntry!()