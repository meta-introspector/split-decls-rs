// Generated macro for Guard (struct)
macro_rules! DepcrateGuard {
() => {
// Module: crate
// Provides: {"Guard"}
// Dependencies: {}
# [doc = " RAII guard which returns DIT to its previous state when dropped."] pub struct Guard < 'a > { # [doc = " DIT implementation."] dit : & 'a Dit , # [doc = " Previous DIT state before it was enabled."] was_enabled : bool , }
};
}
