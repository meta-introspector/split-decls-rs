// Generated macro for is_tty_stdin (function)
macro_rules! Depcrateis_tty_stdin {
() => {
// Module: crate
// Provides: {"is_tty_stdin"}
// Dependencies: {}
# [doc = " Returns true if and only if stdin is believed to be connected to a tty"] # [doc = " or a console."] # [doc = ""] # [doc = " Note that this is now just a wrapper around"] # [doc = " [`std::io::IsTerminal`](https://doc.rust-lang.org/std/io/trait.IsTerminal.html)."] # [doc = " Callers should prefer using the `IsTerminal` trait directly. This routine"] # [doc = " is deprecated and will be removed in the next semver incompatible release."] # [deprecated (since = "0.1.10" , note = "use std::io::IsTerminal instead")] pub fn is_tty_stdin () -> bool { use std :: io :: IsTerminal ; std :: io :: stdin () . is_terminal () }
};
}
