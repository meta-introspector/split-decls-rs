// Generated macro for UseColours (enum)
macro_rules! Depcrate_themeUseColours {
() => {
// Module: crate::theme
// Provides: {"UseColours"}
// Dependencies: {}
# [doc = " Under what circumstances we should display coloured, rather than plain,"] # [doc = " output to the terminal."] # [doc = ""] # [doc = " By default, we want to display the colours when stdout can display them."] # [doc = " Turning them on when output is going to, say, a pipe, would make programs"] # [doc = " such as `grep` or `more` not work properly. So the `Automatic` mode does"] # [doc = " this check and only displays colours when they can be truly appreciated."] # [derive (PartialEq , Eq , Debug , Copy , Clone)] pub enum UseColours { # [doc = " Display them even when output isn’t going to a terminal."] Always , # [doc = " Display them when output is going to a terminal, but not otherwise."] Automatic , # [doc = " Never display them, even when output is going to a terminal."] Never , }
};
}
