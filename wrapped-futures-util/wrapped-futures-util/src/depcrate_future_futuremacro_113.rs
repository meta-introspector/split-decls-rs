// Generated macro for macro_113 (macro)
macro_rules! Depcrate_future_futuremacro_113 {
() => {
// Module: crate::future::future
// Provides: {"macro_113"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`flatten`](super::FutureExt::flatten) method."] Flatten < F > (flatten :: Flatten < F , < F as Future >:: Output >) : Debug + Future + FusedFuture + New [| x : F | flatten :: Flatten :: new (x)] where F : Future) ;
};
}
