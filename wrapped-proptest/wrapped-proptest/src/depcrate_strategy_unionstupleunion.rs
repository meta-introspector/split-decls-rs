// Generated macro for TupleUnion (struct)
macro_rules! Depcrate_strategy_unionsTupleUnion {
() => {
// Module: crate::strategy::unions
// Provides: {"TupleUnion"}
// Dependencies: {}
# [doc = " Similar to `Union`, but internally uses a tuple to hold the strategies."] # [doc = ""] # [doc = " This allows better performance than vanilla `Union` since one does not need"] # [doc = " to resort to boxing and dynamic dispatch to handle heterogeneous"] # [doc = " strategies."] # [doc = ""] # [doc = " The difference between this and `TupleUnion` is that with this, value trees"] # [doc = " for variants that aren't picked at first are generated lazily."] # [must_use = "strategies do nothing unless used"] # [derive (Clone , Copy , Debug)] pub struct TupleUnion < T > (T) ;
};
}
