// Generated macro for macro_16 (macro)
macro_rules! Depcrate_futuremacro_16 {
() => {
// Module: crate::future
// Provides: {"macro_16"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`poll_once()`] function."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct PollOnce < F > { # [pin] f : F , } }
};
}
