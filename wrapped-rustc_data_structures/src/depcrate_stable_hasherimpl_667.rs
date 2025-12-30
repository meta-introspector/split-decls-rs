// Generated macro for impl_667 (impl)
macro_rules! Depcrate_stable_hasherimpl_667 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_667"}
// Dependencies: {}
impl < T1 : StableOrd , T2 : StableOrd > StableOrd for (T1 , T2) { const CAN_USE_UNSTABLE_SORT : bool = T1 :: CAN_USE_UNSTABLE_SORT && T2 :: CAN_USE_UNSTABLE_SORT ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }
};
}
