macro_rules! deps {
    () => {
        IsTerminal!();
        StripStream!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < S > StripStream < S > where S : std :: io :: Write , S : IsTerminal , { # [doc = " Returns `true` if the descriptor/handle refers to a terminal/tty."] # [inline] pub fn is_terminal (& self) -> bool { self . raw . is_terminal () } }
    };
}

impl_114!();