macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        # [cfg (target_os = "unknown")] impl IsTerminal for std :: io :: Stderr { # [inline] fn is_terminal (& self) -> bool { false } }
    };
}

impl_8!()