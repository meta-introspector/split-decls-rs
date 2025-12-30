// Generated macro for impl_60 (impl)
macro_rules! Depcrate_atomicimpl_60 {
() => {
// Module: crate::atomic
// Provides: {"impl_60"}
// Dependencies: {}
impl < T > From < Box < T > > for Owned < T > { # [doc = " Returns a new owned pointer pointing to `b`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the pointer (the `Box`) is not properly aligned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_epoch::Owned;"] # [doc = ""] # [doc = " let o = unsafe { Owned::from_raw(Box::into_raw(Box::new(1234))) };"] # [doc = " ```"] fn from (b : Box < T >) -> Self { unsafe { Self :: from_raw (Box :: into_raw (b)) } } }
};
}
