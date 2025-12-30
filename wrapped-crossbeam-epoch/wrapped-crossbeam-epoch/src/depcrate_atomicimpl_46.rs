// Generated macro for impl_46 (impl)
macro_rules! Depcrate_atomicimpl_46 {
() => {
// Module: crate::atomic
// Provides: {"impl_46"}
// Dependencies: {}
impl < 'g , T : ? Sized + Pointable > From < Shared < 'g , T > > for Atomic < T > { # [doc = " Returns a new atomic pointer pointing to `ptr`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_epoch::{Atomic, Shared};"] # [doc = ""] # [doc = " let a = Atomic::<i32>::from(Shared::<i32>::null());"] # [doc = " ```"] fn from (ptr : Shared < 'g , T >) -> Self { Self :: from_ptr (ptr . data) } }
};
}
