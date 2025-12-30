// Generated macro for impl_890 (impl)
macro_rules! Depcrate_strategy_mapimpl_890 {
() => {
// Module: crate::strategy::map
// Provides: {"impl_890"}
// Dependencies: {}
impl < S , O > MapInto < S , O > { # [doc = " Construct a `MapInto` mapper from an `S` strategy into a strategy"] # [doc = " producing `O`s."] pub (super) fn new (source : S) -> Self { Self { source , output : PhantomData , } } }
};
}
