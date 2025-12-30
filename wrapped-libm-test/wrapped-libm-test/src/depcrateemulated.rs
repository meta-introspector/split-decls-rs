// Generated macro for emulated (function)
macro_rules! Depcrateemulated {
() => {
// Module: crate
// Provides: {"emulated"}
// Dependencies: {}
# [doc = " True if `EMULATED` is set and nonempty. Used to determine how many iterations to run."] pub const fn emulated () -> bool { match option_env ! ("EMULATED") { Some (s) if s . is_empty () => false , None => false , Some (_) => true , } }
};
}
