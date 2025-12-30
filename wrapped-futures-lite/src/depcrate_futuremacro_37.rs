// Generated macro for macro_37 (macro)
macro_rules! Depcrate_futuremacro_37 {
() => {
// Module: crate::future
// Provides: {"macro_37"}
// Dependencies: {}
pin_project ! { # [doc = " [`Future`] for the [`fuse`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Fuse < Fut > { # [pin] inner : Option < Fut >, } }
};
}
