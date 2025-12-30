// Generated macro for impl_205 (impl)
macro_rules! Depcrate_internalimpl_205 {
() => {
// Module: crate::internal
// Provides: {"impl_205"}
// Dependencies: {}
impl Mode for Emit { type Output < T > = T ; # [inline (always)] fn bind < T , F : FnOnce () -> T > (f : F) -> Self :: Output < T > { f () } # [inline (always)] fn map < T , U , F : FnOnce (T) -> U > (x : Self :: Output < T > , f : F) -> Self :: Output < U > { f (x) } # [inline (always)] fn combine < T , U , V , F : FnOnce (T , U) -> V > (x : Self :: Output < T > , y : Self :: Output < U > , f : F ,) -> Self :: Output < V > { f (x , y) } }
};
}
