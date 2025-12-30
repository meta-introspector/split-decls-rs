// Generated macro for macro_116 (macro)
macro_rules! Depcrate_future_futuremacro_116 {
() => {
// Module: crate::future::future
// Provides: {"macro_116"}
// Dependencies: {}
delegate_all ! (# [doc = " Future for the [`map`](super::FutureExt::map) method."] Map < Fut , F > (map :: Map < Fut , F >) : Debug + Future + FusedFuture + New [| x : Fut , f : F | map :: Map :: new (x , f)]) ;
};
}
