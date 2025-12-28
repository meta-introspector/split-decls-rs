macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! macro_31 {
    () => {
        deps!();
        code ! { HasConstant (2) (Field , 0) }
    };
}

macro_31!();