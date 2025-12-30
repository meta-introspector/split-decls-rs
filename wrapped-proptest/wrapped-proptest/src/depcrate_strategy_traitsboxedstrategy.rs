// Generated macro for BoxedStrategy (struct)
macro_rules! Depcrate_strategy_traitsBoxedStrategy {
() => {
// Module: crate::strategy::traits
// Provides: {"BoxedStrategy"}
// Dependencies: {}
# [doc = " A boxed `Strategy` trait object as produced by `Strategy::boxed()`."] # [doc = ""] # [doc = " Strategies of this type afford cheap shallow cloning via reference"] # [doc = " counting by using an `Arc` internally."] # [derive (Debug)] # [must_use = "strategies do nothing unless used"] pub struct BoxedStrategy < T > (Arc < dyn Strategy < Value = T , Tree = BoxedVT < T > > >) ;
};
}
