// Generated macro for macro_131 (macro)
macro_rules! Depcrate_future_future_catch_unwindmacro_131 {
() => {
// Module: crate::future::future::catch_unwind
// Provides: {"macro_131"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`catch_unwind`](super::FutureExt::catch_unwind) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct CatchUnwind < Fut > { # [pin] future : Fut , } }
};
}
