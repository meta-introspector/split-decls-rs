macro_rules! deps {
    () => {
        WinconStream!();
        IsTerminal!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < S > WinconStream < S > where S : anstyle_wincon :: WinconStream , S : IsTerminal , { # [doc = " Returns `true` if the descriptor/handle refers to a terminal/tty."] # [inline] pub fn is_terminal (& self) -> bool { self . raw . is_terminal () } }
    };
}

impl_126!();