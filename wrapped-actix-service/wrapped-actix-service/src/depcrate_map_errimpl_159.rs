// Generated macro for impl_159 (impl)
macro_rules! Depcrate_map_errimpl_159 {
() => {
// Module: crate::map_err
// Provides: {"impl_159"}
// Dependencies: {}
impl < A , Req , F , E > MapErrFuture < A , Req , F , E > where A : Service < Req > , F : Fn (A :: Error) -> E , { fn new (fut : A :: Future , f : F) -> Self { MapErrFuture { f , fut } } }
};
}
