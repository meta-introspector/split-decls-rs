// Generated macro for macro_34 (macro)
macro_rules! Depcrate_futuremacro_34 {
() => {
// Module: crate::future
// Provides: {"macro_34"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`or()`] function and the [`FutureExt::or()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Or < F1 , F2 > { # [pin] future1 : F1 , # [pin] future2 : F2 , } }
};
}
