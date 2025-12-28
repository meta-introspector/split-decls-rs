macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        # [cfg (target_os = "unknown")] impl < 'a > IsTerminal for std :: io :: StderrLock < 'a > { # [inline] fn is_terminal (& self) -> bool { false } }
    };
}

impl_11!()