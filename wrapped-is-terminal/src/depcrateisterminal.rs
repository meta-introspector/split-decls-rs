// Generated macro for IsTerminal (trait)
macro_rules! DepcrateIsTerminal {
() => {
// Module: crate
// Provides: {"IsTerminal"}
// Dependencies: {}
# [doc = " Extension trait to check whether something is a terminal."] pub trait IsTerminal { # [doc = " Returns true if this is a terminal."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use is_terminal::IsTerminal;"] # [doc = ""] # [doc = " if std::io::stdout().is_terminal() {"] # [doc = "     println!(\"stdout is a terminal\")"] # [doc = " }"] # [doc = " ```"] fn is_terminal (& self) -> bool ; }
};
}
