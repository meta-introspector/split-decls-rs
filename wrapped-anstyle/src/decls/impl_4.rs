macro_rules! deps {
    () => {
        AnsiColor!();
        Color!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl From < AnsiColor > for Color { # [inline] fn from (inner : AnsiColor) -> Self { Self :: Ansi (inner) } }
    };
}

impl_4!();