macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! LabelPtr {
    () => {
        deps!();
        pub type LabelPtr = AstPtr < ast :: Label > ;
    };
}

LabelPtr!();