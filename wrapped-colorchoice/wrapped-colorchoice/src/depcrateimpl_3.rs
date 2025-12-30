// Generated macro for impl_3 (impl)
macro_rules! Depcrateimpl_3 {
() => {
// Module: crate
// Provides: {"impl_3"}
// Dependencies: {}
impl ColorChoice { # [doc = " Get the current [`ColorChoice`] state"] pub fn global () -> Self { USER . get () } # [doc = " Override the detected [`ColorChoice`]"] pub fn write_global (self) { USER . set (self) ; } }
};
}
