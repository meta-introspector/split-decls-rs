// Generated macro for ModifierDiff (struct)
macro_rules! DepcrateModifierDiff {
() => {
// Module: crate
// Provides: {"ModifierDiff"}
// Dependencies: {}
# [doc = " The `ModifierDiff` struct is used to calculate the difference between two `Modifier`"] # [doc = " values. This is useful when updating the terminal display, as it allows for more"] # [doc = " efficient updates by only sending the necessary changes."] struct ModifierDiff { from : Modifier , to : Modifier , }
};
}
