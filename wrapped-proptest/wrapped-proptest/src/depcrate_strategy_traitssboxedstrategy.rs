// Generated macro for SBoxedStrategy (struct)
macro_rules! Depcrate_strategy_traitsSBoxedStrategy {
() => {
// Module: crate::strategy::traits
// Provides: {"SBoxedStrategy"}
// Dependencies: {}
# [doc = " A boxed `Strategy` trait object which is also `Sync` and"] # [doc = " `Send`, as produced by `Strategy::sboxed()`."] # [doc = ""] # [doc = " Strategies of this type afford cheap shallow cloning via reference"] # [doc = " counting by using an `Arc` internally."] # [derive (Debug)] # [must_use = "strategies do nothing unless used"] pub struct SBoxedStrategy < T > (Arc < dyn Strategy < Value = T , Tree = BoxedVT < T > > + Sync + Send > ,) ;
};
}
