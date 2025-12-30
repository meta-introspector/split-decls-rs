// Generated macro for macro_120 (macro)
macro_rules! Depcrate_future_futuremacro_120 {
() => {
// Module: crate::future::future
// Provides: {"macro_120"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`inspect`](FutureExt::inspect) method."] Inspect < Fut , F > (map :: Map < Fut , InspectFn < F >>) : Debug + Future + FusedFuture + New [| x : Fut , f : F | map :: Map :: new (x , inspect_fn (f))]) ;
};
}
