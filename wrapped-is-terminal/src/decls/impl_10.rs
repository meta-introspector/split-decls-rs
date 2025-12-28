macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        # [cfg (target_os = "unknown")] impl < 'a > IsTerminal for std :: io :: StdoutLock < 'a > { # [inline] fn is_terminal (& self) -> bool { false } }
    };
}

impl_10!();