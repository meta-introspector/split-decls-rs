// Generated macro for macro_208 (macro)
macro_rules! Depcrate_future_try_future_into_futuremacro_208 {
() => {
// Module: crate::future::try_future::into_future
// Provides: {"macro_208"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`into_future`](super::TryFutureExt::into_future) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct IntoFuture < Fut > { # [pin] future : Fut , } }
};
}
