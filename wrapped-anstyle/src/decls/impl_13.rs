macro_rules! deps {
    () => {
        Ansi256Color!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl From < u8 > for Ansi256Color { # [inline] fn from (inner : u8) -> Self { Self (inner) } }
    };
}

impl_13!();