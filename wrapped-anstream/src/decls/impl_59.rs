macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < T : IsTerminal + ? Sized > IsTerminal for Box < T > { # [inline] fn is_terminal (& self) -> bool { (* * self) . is_terminal () } }
    };
}

impl_59!()