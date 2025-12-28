macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl IsTerminal for dyn std :: io :: Write + Send + Sync { # [inline] fn is_terminal (& self) -> bool { false } }
    };
}

impl_66!()