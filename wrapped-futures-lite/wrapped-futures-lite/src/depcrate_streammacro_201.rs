// Generated macro for macro_201 (macro)
macro_rules! Depcrate_streammacro_201 {
() => {
// Module: crate::stream
// Provides: {"macro_201"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`StreamExt::for_each()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct ForEachFuture < S , F > { # [pin] stream : S , f : F , } }
};
}
