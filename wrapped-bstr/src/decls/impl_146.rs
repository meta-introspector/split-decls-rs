macro_rules! deps {
    () => {
        BufReadExt!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < B : io :: BufRead > BufReadExt for B { }
    };
}

impl_146!();