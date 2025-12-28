macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl IsTerminal for std :: fs :: File { # [inline] fn is_terminal (& self) -> bool { is_terminal_polyfill :: IsTerminal :: is_terminal (self) } }
    };
}

impl_68!();