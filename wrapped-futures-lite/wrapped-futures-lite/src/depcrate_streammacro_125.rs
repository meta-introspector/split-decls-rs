// Generated macro for macro_125 (macro)
macro_rules! Depcrate_streammacro_125 {
() => {
// Module: crate::stream
// Provides: {"macro_125"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`StreamExt::try_collect()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryCollectFuture < S , C > { # [pin] stream : S , items : C , } }
};
}
