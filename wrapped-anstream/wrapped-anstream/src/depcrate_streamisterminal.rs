// Generated macro for IsTerminal (trait)
macro_rules! Depcrate_streamIsTerminal {
() => {
// Module: crate::stream
// Provides: {"IsTerminal"}
// Dependencies: {}
# [doc = " Trait to determine if a descriptor/handle refers to a terminal/tty."] pub trait IsTerminal : private :: Sealed { # [doc = " Returns `true` if the descriptor/handle refers to a terminal/tty."] fn is_terminal (& self) -> bool ; }
};
}
