macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        # [cfg (target_os = "unknown")] impl IsTerminal for std :: process :: ChildStdin { # [inline] fn is_terminal (& self) -> bool { false } }
    };
}

impl_13!()