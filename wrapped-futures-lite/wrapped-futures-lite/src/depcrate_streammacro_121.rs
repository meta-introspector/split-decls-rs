// Generated macro for macro_121 (macro)
macro_rules! Depcrate_streammacro_121 {
() => {
// Module: crate::stream
// Provides: {"macro_121"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`StreamExt::count()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct CountFuture < S : ? Sized > { count : usize , # [pin] stream : S , } }
};
}
