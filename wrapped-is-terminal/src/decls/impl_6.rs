macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        # [cfg (target_os = "unknown")] impl IsTerminal for std :: io :: Stdin { # [inline] fn is_terminal (& self) -> bool { false } }
    };
}

impl_6!();