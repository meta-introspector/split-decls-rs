// Generated macro for impl_28 (impl)
macro_rules! Depcrate_chainimpl_28 {
() => {
// Module: crate::chain
// Provides: {"impl_28"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl Default for Chain < '_ > { fn default () -> Self { Chain { state : ChainState :: Buffered { rest : Vec :: new () . into_iter () , } , } } }
};
}
