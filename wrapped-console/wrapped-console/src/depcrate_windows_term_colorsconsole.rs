// Generated macro for Console (struct)
macro_rules! Depcrate_windows_term_colorsConsole {
() => {
// Module: crate::windows_term::colors
// Provides: {"Console"}
// Dependencies: {}
# [doc = " A Windows console."] # [doc = ""] # [doc = " This represents a very limited set of functionality available to a Windows"] # [doc = " console. In particular, it can only change text attributes such as color"] # [doc = " and intensity. This may grow over time. If you need more routines, please"] # [doc = " file an issue and/or PR."] # [doc = ""] # [doc = " There is no way to \"write\" to this console. Simply write to"] # [doc = " stdout or stderr instead, while interleaving instructions to the console"] # [doc = " to change text attributes."] # [doc = ""] # [doc = " A common pitfall when using a console is to forget to flush writes to"] # [doc = " stdout before setting new text attributes."] # [derive (Debug)] pub (crate) struct Console { kind : HandleKind , start_attr : TextAttributes , cur_attr : TextAttributes , }
};
}
