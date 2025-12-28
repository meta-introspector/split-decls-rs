macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        # [cfg (target_os = "unknown")] impl < 'a > IsTerminal for std :: fs :: File { # [inline] fn is_terminal (& self) -> bool { false } }
    };
}

impl_12!()