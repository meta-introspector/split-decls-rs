// Generated macro for impl_162 (impl)
macro_rules! Depcrate_map_errimpl_162 {
() => {
// Module: crate::map_err
// Provides: {"impl_162"}
// Dependencies: {}
impl < SF , Req , F , E > MapErrServiceFactory < SF , Req , F , E > where SF : ServiceFactory < Req > , F : Fn (SF :: Error) -> E + Clone , { # [doc = " Create new `MapErr` new service instance"] pub (crate) fn new (a : SF , f : F) -> Self { Self { a , f , e : PhantomData , } } }
};
}
