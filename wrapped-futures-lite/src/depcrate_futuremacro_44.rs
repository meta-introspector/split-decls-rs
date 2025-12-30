// Generated macro for macro_44 (macro)
macro_rules! Depcrate_futuremacro_44 {
() => {
// Module: crate::future
// Provides: {"macro_44"}
// Dependencies: {}
# [cfg (feature = "std")] pin_project ! { # [doc = " Future for the [`FutureExt::catch_unwind()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct CatchUnwind < F > { # [pin] inner : F , } }
};
}
