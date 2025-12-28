macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl IsTerminal for std :: io :: StderrLock < '_ > { # [inline] fn is_terminal (& self) -> bool { is_terminal_polyfill :: IsTerminal :: is_terminal (self) } }
    };
}

impl_63!()