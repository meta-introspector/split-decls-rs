macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        # [cfg (target_os = "unknown")] impl IsTerminal for std :: io :: Stdout { # [inline] fn is_terminal (& self) -> bool { false } }
    };
}

impl_7!()