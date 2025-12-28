macro_rules! deps {
    () => {
        AnsiColor!();
        Ansi256Color!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl From < AnsiColor > for Ansi256Color { # [inline] fn from (inner : AnsiColor) -> Self { Self :: from_ansi (inner) } }
    };
}

impl_14!()