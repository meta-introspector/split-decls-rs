macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        # [cfg (target_os = "unknown")] impl < 'a > IsTerminal for std :: io :: StdinLock < 'a > { # [inline] fn is_terminal (& self) -> bool { false } }
    };
}

impl_9!()