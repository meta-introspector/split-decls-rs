// Generated macro for impl_1669 (impl)
macro_rules! Depcrate_syncimpl_1669 {
() => {
// Module: crate::sync
// Provides: {"impl_1669"}
// Dependencies: {}
impl < T : ? Sized , A : Allocator + Clone > UniqueArc < T , A > { # [doc = " Creates a new weak reference to the `UniqueArc`."] # [doc = ""] # [doc = " Attempting to upgrade this weak reference will fail before the `UniqueArc` has been converted"] # [doc = " to a [`Arc`] using [`UniqueArc::into_arc`]."] # [unstable (feature = "unique_rc_arc" , issue = "112566")] # [must_use] pub fn downgrade (this : & Self) -> Weak < T , A > { let old_size = unsafe { (* this . ptr . as_ptr ()) . weak . fetch_add (1 , Relaxed) } ; if old_size > MAX_REFCOUNT { abort () ; } Weak { ptr : this . ptr , alloc : this . alloc . clone () } } }
};
}
