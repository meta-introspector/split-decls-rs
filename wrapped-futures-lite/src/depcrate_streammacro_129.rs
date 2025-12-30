// Generated macro for macro_129 (macro)
macro_rules! Depcrate_streammacro_129 {
() => {
// Module: crate::stream
// Provides: {"macro_129"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`StreamExt::fold()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct FoldFuture < S , F , T > { # [pin] stream : S , f : F , acc : Option < T >, } }
};
}
