// Generated macro for macro_123 (macro)
macro_rules! Depcrate_streammacro_123 {
() => {
// Module: crate::stream
// Provides: {"macro_123"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`StreamExt::collect()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct CollectFuture < S , C > { # [pin] stream : S , collection : C , } }
};
}
