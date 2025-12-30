// Generated macro for impl_158 (impl)
macro_rules! Depcrate_utilimpl_158 {
() => {
// Module: crate::util
// Provides: {"impl_158"}
// Dependencies: {}
# [cfg (any (feature = "portable-atomic" , target_has_atomic = "8"))] impl < BUS > AtomicCell < BUS > { # [doc = " Create a new `AtomicCell`"] pub fn new (bus : BUS) -> Self { Self { bus : UnsafeCell :: new (bus) , busy : AtomicBool :: from (false) , } } }
};
}
