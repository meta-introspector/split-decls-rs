macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl IsTerminal for dyn std :: io :: Write { # [inline] fn is_terminal (& self) -> bool { false } }
    };
}

impl_64!();