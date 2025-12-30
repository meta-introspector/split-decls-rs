// Generated macro for impl_3 (impl)
macro_rules! Depcrateimpl_3 {
() => {
// Module: crate
// Provides: {"impl_3"}
// Dependencies: {}
impl Color { # [doc = " Set the user selection on `colorchoice`"] pub fn write_global (& self) { self . as_choice () . write_global () ; } # [doc = " Get the user's selection"] pub fn as_choice (& self) -> colorchoice :: ColorChoice { match self . color { ColorChoice :: Auto => colorchoice :: ColorChoice :: Auto , ColorChoice :: Always => colorchoice :: ColorChoice :: Always , ColorChoice :: Never => colorchoice :: ColorChoice :: Never , } } }
};
}
