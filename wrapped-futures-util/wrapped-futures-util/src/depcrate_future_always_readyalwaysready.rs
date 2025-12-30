// Generated macro for AlwaysReady (struct)
macro_rules! Depcrate_future_always_readyAlwaysReady {
() => {
// Module: crate::future::always_ready
// Provides: {"AlwaysReady"}
// Dependencies: {}
# [doc = " Future for the [`always_ready`](always_ready()) function."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct AlwaysReady < T , F : Fn () -> T > (F) ;
};
}
