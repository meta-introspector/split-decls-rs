macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl IsTerminal for Vec < u8 > { # [inline] fn is_terminal (& self) -> bool { false } }
    };
}

impl_67!();