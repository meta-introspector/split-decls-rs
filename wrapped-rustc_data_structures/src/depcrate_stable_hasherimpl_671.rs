// Generated macro for impl_671 (impl)
macro_rules! Depcrate_stable_hasherimpl_671 {
() => {
// Module: crate::stable_hasher
// Provides: {"impl_671"}
// Dependencies: {}
impl < T1 : StableOrd , T2 : StableOrd , T3 : StableOrd , T4 : StableOrd > StableOrd for (T1 , T2 , T3 , T4) { const CAN_USE_UNSTABLE_SORT : bool = T1 :: CAN_USE_UNSTABLE_SORT && T2 :: CAN_USE_UNSTABLE_SORT && T3 :: CAN_USE_UNSTABLE_SORT && T4 :: CAN_USE_UNSTABLE_SORT ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }
};
}
