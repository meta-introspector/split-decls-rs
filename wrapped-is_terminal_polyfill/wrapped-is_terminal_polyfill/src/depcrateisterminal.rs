// Generated macro for IsTerminal (trait)
macro_rules! DepcrateIsTerminal {
() => {
// Module: crate
// Provides: {"IsTerminal"}
// Dependencies: {}
# [doc = " Trait to determine if a descriptor/handle refers to a terminal/tty."] pub trait IsTerminal : sealed :: Sealed { # [doc = " Returns `true` if the descriptor/handle refers to a terminal/tty."] # [doc = ""] # [doc = " On platforms where Rust does not know how to detect a terminal yet, this will return"] # [doc = " `false`. This will also return `false` if an unexpected error occurred, such as from"] # [doc = " passing an invalid file descriptor."] # [doc = ""] # [doc = " # Platform-specific behavior"] # [doc = ""] # [doc = " On Windows, in addition to detecting consoles, this currently uses some heuristics to"] # [doc = " detect older msys/cygwin/mingw pseudo-terminals based on device name: devices with names"] # [doc = " starting with `msys-` or `cygwin-` and ending in `-pty` will be considered terminals."] # [doc = " Note that this [may change in the future][changes]."] # [doc = ""] # [doc = " [changes]: std::io#platform-specific-behavior"] fn is_terminal (& self) -> bool ; }
};
}
