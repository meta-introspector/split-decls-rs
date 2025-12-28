macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl From < u8 > for State { fn from (s : u8) -> Self { Self { state : s } } }
    };
}

impl_119!();