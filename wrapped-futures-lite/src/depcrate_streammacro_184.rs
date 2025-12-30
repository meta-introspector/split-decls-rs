// Generated macro for macro_184 (macro)
macro_rules! Depcrate_streammacro_184 {
() => {
// Module: crate::stream
// Provides: {"macro_184"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`StreamExt::last()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct LastFuture < S : Stream > { # [pin] stream : S , last : Option < S :: Item >, } }
};
}
