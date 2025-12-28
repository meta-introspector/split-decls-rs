macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < T : IsTerminal + ? Sized > IsTerminal for & T { # [inline] fn is_terminal (& self) -> bool { (* * self) . is_terminal () } }
    };
}

impl_57!();