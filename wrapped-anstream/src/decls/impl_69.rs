macro_rules! deps {
    () => {
        Buffer!();
        IsTerminal!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        # [allow (deprecated)] impl IsTerminal for crate :: Buffer { # [inline] fn is_terminal (& self) -> bool { false } }
    };
}

impl_69!()