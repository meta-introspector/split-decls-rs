macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl IsTerminal for std :: io :: StdoutLock < '_ > { # [inline] fn is_terminal (& self) -> bool { is_terminal_polyfill :: IsTerminal :: is_terminal (self) } }
    };
}

impl_61!()