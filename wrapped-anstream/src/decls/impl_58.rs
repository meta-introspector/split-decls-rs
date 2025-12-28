macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < T : IsTerminal + ? Sized > IsTerminal for & mut T { # [inline] fn is_terminal (& self) -> bool { (* * self) . is_terminal () } }
    };
}

impl_58!()