// Generated macro for macro_118 (macro)
macro_rules! Depcrate_future_futuremacro_118 {
() => {
// Module: crate::future::future
// Provides: {"macro_118"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`map_into`](FutureExt::map_into) combinator."] MapInto < Fut , T > (Map < Fut , IntoFn < T >>) : Debug + Future + FusedFuture + New [| x : Fut | Map :: new (x , into_fn ())]) ;
};
}
