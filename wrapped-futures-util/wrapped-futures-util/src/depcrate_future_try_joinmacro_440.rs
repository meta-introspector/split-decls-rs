// Generated macro for macro_440 (macro)
macro_rules! Depcrate_future_try_joinmacro_440 {
() => {
// Module: crate::future::try_join
// Provides: {"macro_440"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`try_join`](super::TryFutureExt::try_join) method."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryJoin < Fut1 : TryFuture , Fut2 : TryFuture > { # [pin] fut1 : TryMaybeDone < Fut1 >, # [pin] fut2 : TryMaybeDone < Fut2 > } }
};
}
