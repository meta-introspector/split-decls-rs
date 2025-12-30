// Generated macro for impl_178 (impl)
macro_rules! Depcrate_map_init_errimpl_178 {
() => {
// Module: crate::map_init_err
// Provides: {"impl_178"}
// Dependencies: {}
impl < A , F , Req , E > MapInitErrFuture < A , F , Req , E > where A : ServiceFactory < Req > , F : Fn (A :: InitError) -> E , { fn new (fut : A :: Future , f : F) -> Self { MapInitErrFuture { f , fut } } }
};
}
