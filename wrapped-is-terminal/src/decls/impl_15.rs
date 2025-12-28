macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [cfg (target_os = "unknown")] impl IsTerminal for std :: process :: ChildStderr { # [inline] fn is_terminal (& self) -> bool { false } }
    };
}

impl_15!()