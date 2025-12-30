// Generated macro for macro_354 (macro)
macro_rules! Depcrate_arbitrary__alloc_opsmacro_354 {
() => {
// Module: crate::arbitrary::_alloc::ops
// Provides: {"macro_354"}
// Dependencies: {}
# [cfg (feature = "unstable")] arbitrary ! ([Y : Arbitrary , R : Arbitrary] CoroutineState < Y , R >, TupleUnion < (WA < SMapped < Y , Self >>, WA < SMapped < R , Self >>) >, product_type ! [Y :: Parameters , R :: Parameters] ; args => { let product_unpack ! [y , r] = args ; prop_oneof ! [static_map (any_with ::< Y > (y) , CoroutineState :: Yielded) , static_map (any_with ::< R > (r) , CoroutineState :: Complete)] }) ;
};
}
