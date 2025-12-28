macro_rules! deps {
    () => {
        Ansi256Color!();
        Color!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl From < Ansi256Color > for Color { # [inline] fn from (inner : Ansi256Color) -> Self { Self :: Ansi256 (inner) } }
    };
}

impl_5!()