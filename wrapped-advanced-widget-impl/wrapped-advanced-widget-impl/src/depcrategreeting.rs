// Generated macro for Greeting (struct)
macro_rules! DepcrateGreeting {
() => {
// Module: crate
// Provides: {"Greeting"}
// Dependencies: {}
# [doc = " An ephemeral greeting widget."] # [doc = ""] # [doc = " This widget is implemented on the type itself, which means that it is consumed when it is"] # [doc = " rendered. This is useful for widgets that are cheap to create, don't need to be reused, and"] # [doc = " don't need to store any state between renders. This is the simplest way to implement a widget in"] # [doc = " Ratatui, but in most cases, it is better to implement the `Widget` trait on a reference to the"] # [doc = " type, as shown in the other examples below."] # [doc = ""] # [doc = " This was the way most widgets were implemented in Ratatui before `Widget` was implemented on"] # [doc = " references in [PR #903] (merged in Ratatui 0.26.0)."] # [doc = ""] # [doc = " [PR #903]: https://github.com/ratatui/ratatui/pull/903"] struct Greeting { name : String , }
};
}
