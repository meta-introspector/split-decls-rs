// Generated macro for impl_155 (impl)
macro_rules! Depcrate_internalimpl_155 {
() => {
// Module: crate::internal
// Provides: {"impl_155"}
// Dependencies: {}
impl SealedBag { # [doc = " Checks if it is safe to drop the bag w.r.t. the given global epoch."] fn is_expired (& self , global_epoch : Epoch) -> bool { global_epoch . wrapping_sub (self . epoch) >= 2 } }
};
}
