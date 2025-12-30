// Generated macro for impl_1293 (impl)
macro_rules! Depcrate_tupleimpl_1293 {
() => {
// Module: crate::tuple
// Provides: {"impl_1293"}
// Dependencies: {}
impl < T > TupleValueTree < T > { # [doc = " Create a new `TupleValueTree` wrapping `inner`."] # [doc = ""] # [doc = " It only makes sense for `inner` to be a tuple of an arity for which the"] # [doc = " type implements `ValueTree`."] pub fn new (inner : T) -> Self { TupleValueTree { tree : inner , shrinker : 0 , prev_shrinker : None , } } }
};
}
