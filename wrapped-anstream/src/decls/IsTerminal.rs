macro_rules! IsTerminal {
    () => {
        # [doc = " Trait to determine if a descriptor/handle refers to a terminal/tty."] pub trait IsTerminal : private :: Sealed { # [doc = " Returns `true` if the descriptor/handle refers to a terminal/tty."] fn is_terminal (& self) -> bool ; }
    };
}

IsTerminal!();