// Generated macro for ColorChoice (enum)
macro_rules! DepcrateColorChoice {
() => {
// Module: crate
// Provides: {"ColorChoice"}
// Dependencies: {}
# [doc = " Selection for overriding color output"] # [allow (clippy :: exhaustive_enums)] # [derive (Copy , Clone , Debug , PartialEq , Eq , Default)] pub enum ColorChoice { # [doc = " Use colors if the output device appears to support them"] # [default] Auto , # [doc = " Like `Always`, except it never tries to use anything other than emitting ANSI"] # [doc = " color codes."] AlwaysAnsi , # [doc = " Try very hard to emit colors."] # [doc = ""] # [doc = " This includes emitting ANSI colors on Windows if the console API is unavailable."] Always , # [doc = " Never emit colors."] Never , }
};
}
