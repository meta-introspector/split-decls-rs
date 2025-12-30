// Generated macro for impl_166 (impl)
macro_rules! Depcrate_map_errimpl_166 {
() => {
// Module: crate::map_err
// Provides: {"impl_166"}
// Dependencies: {}
impl < SF , Req , F , E > MapErrServiceFuture < SF , Req , F , E > where SF : ServiceFactory < Req > , F : Fn (SF :: Error) -> E , { fn new (fut : SF :: Future , mapper : F) -> Self { MapErrServiceFuture { fut , mapper } } }
};
}
