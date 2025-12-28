macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [cfg (target_os = "unknown")] impl IsTerminal for std :: process :: ChildStdout { # [inline] fn is_terminal (& self) -> bool { false } }
    };
}

impl_14!()