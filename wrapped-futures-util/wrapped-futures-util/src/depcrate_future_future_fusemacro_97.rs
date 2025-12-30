// Generated macro for macro_97 (macro)
macro_rules! Depcrate_future_future_fusemacro_97 {
() => {
// Module: crate::future::future::fuse
// Provides: {"macro_97"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`fuse`](super::FutureExt::fuse) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Fuse < Fut > { # [pin] inner : Option < Fut >, } }
};
}
