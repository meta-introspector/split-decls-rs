macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! macro_146 {
    () => {
        deps!();
        code ! { HasConstant (2) (Field , 0) }
    };
}

macro_146!();