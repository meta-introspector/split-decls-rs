// Generated macro for macro_20 (macro)
macro_rules! Depcrate_futuremacro_20 {
() => {
// Module: crate::future
// Provides: {"macro_20"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`poll_fn()`] function."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct PollFn < F > { f : F , } }
};
}
