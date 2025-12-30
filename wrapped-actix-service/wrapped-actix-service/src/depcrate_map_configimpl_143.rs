// Generated macro for impl_143 (impl)
macro_rules! Depcrate_map_configimpl_143 {
() => {
// Module: crate::map_config
// Provides: {"impl_143"}
// Dependencies: {}
impl < SF , Req , F , Cfg > MapConfig < SF , Req , F , Cfg > { # [doc = " Create new `MapConfig` combinator"] pub (crate) fn new (factory : SF , cfg_mapper : F) -> Self where SF : ServiceFactory < Req > , F : Fn (Cfg) -> SF :: Config , { Self { factory , cfg_mapper , e : PhantomData , } } }
};
}
