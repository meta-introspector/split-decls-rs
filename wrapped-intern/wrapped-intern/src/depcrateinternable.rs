// Generated macro for Internable (trait)
macro_rules! DepcrateInternable {
() => {
// Module: crate
// Provides: {"Internable"}
// Dependencies: {}
pub trait Internable : Hash + Eq + 'static { fn storage () -> & 'static InternStorage < Self > ; }
};
}
