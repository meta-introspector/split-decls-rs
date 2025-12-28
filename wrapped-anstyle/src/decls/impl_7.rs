macro_rules! deps {
    () => {
        Color!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl From < u8 > for Color { # [inline] fn from (inner : u8) -> Self { Self :: Ansi256 (inner . into ()) } }
    };
}

impl_7!();