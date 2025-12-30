// Generated macro for macro_42 (macro)
macro_rules! Depcrate_futuremacro_42 {
() => {
// Module: crate::future
// Provides: {"macro_42"}
// Dependencies: {}
# [cfg (feature = "race")] pin_project ! { # [doc = " Future for the [`race()`] function and the [`FutureExt::race()`] method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Race < F1 , F2 > { # [pin] future1 : F1 , # [pin] future2 : F2 , rng : Rng , } }
};
}
