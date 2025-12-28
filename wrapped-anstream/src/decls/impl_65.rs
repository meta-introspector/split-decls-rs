macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl IsTerminal for dyn std :: io :: Write + Send { # [inline] fn is_terminal (& self) -> bool { false } }
    };
}

impl_65!();