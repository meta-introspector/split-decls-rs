// Generated macro for is_tty_stdout (function)
macro_rules! Depcrateis_tty_stdout {
() => {
// Module: crate
// Provides: {"is_tty_stdout"}
// Dependencies: {}
# [doc = " Returns true if and only if stdout is believed to be connected to a tty"] # [doc = " or a console."] # [doc = ""] # [doc = " This is useful for when you want your command line program to produce"] # [doc = " different output depending on whether it's printing directly to a user's"] # [doc = " terminal or whether it's being redirected somewhere else. For example,"] # [doc = " implementations of `ls` will often show one item per line when stdout is"] # [doc = " redirected, but will condensed output when printing to a tty."] # [doc = ""] # [doc = " Note that this is now just a wrapper around"] # [doc = " [`std::io::IsTerminal`](https://doc.rust-lang.org/std/io/trait.IsTerminal.html)."] # [doc = " Callers should prefer using the `IsTerminal` trait directly. This routine"] # [doc = " is deprecated and will be removed in the next semver incompatible release."] # [deprecated (since = "0.1.10" , note = "use std::io::IsTerminal instead")] pub fn is_tty_stdout () -> bool { use std :: io :: IsTerminal ; std :: io :: stdout () . is_terminal () }
};
}
