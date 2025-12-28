macro_rules! deps {
    () => {
        IsTerminal!();
        Stderr!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl IsTerminal for std :: io :: Stderr { # [inline] fn is_terminal (& self) -> bool { is_terminal_polyfill :: IsTerminal :: is_terminal (self) } }
    };
}

impl_62!();