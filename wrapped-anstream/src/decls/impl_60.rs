macro_rules! deps {
    () => {
        IsTerminal!();
        Stdout!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl IsTerminal for std :: io :: Stdout { # [inline] fn is_terminal (& self) -> bool { is_terminal_polyfill :: IsTerminal :: is_terminal (self) } }
    };
}

impl_60!();