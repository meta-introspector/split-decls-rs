// Generated macro for impl_147 (impl)
macro_rules! Depcrate_map_configimpl_147 {
() => {
// Module: crate::map_config
// Provides: {"impl_147"}
// Dependencies: {}
impl < SF , Cfg , Req > UnitConfig < SF , Cfg , Req > where SF : ServiceFactory < Req , Config = () > , { # [doc = " Create new `UnitConfig` combinator"] pub (crate) fn new (factory : SF) -> Self { Self { factory , _phantom : PhantomData , } } }
};
}
