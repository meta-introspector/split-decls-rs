// Generated macro for impl_1317 (impl)
macro_rules! Depcrate_rcimpl_1317 {
() => {
// Module: crate::rc
// Provides: {"impl_1317"}
// Dependencies: {}
impl < T : ? Sized , A : Allocator + Clone > UniqueRc < T , A > { # [doc = " Creates a new weak reference to the `UniqueRc`."] # [doc = ""] # [doc = " Attempting to upgrade this weak reference will fail before the `UniqueRc` has been converted"] # [doc = " to a [`Rc`] using [`UniqueRc::into_rc`]."] # [unstable (feature = "unique_rc_arc" , issue = "112566")] pub fn downgrade (this : & Self) -> Weak < T , A > { unsafe { this . ptr . as_ref () . inc_weak () ; } Weak { ptr : this . ptr , alloc : this . alloc . clone () } } }
};
}
