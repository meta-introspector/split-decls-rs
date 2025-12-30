// Generated macro for impl_1668 (impl)
macro_rules! Depcrate_syncimpl_1668 {
() => {
// Module: crate::sync
// Provides: {"impl_1668"}
// Dependencies: {}
impl < T : ? Sized , A : Allocator > UniqueArc < T , A > { # [doc = " Converts the `UniqueArc` into a regular [`Arc`]."] # [doc = ""] # [doc = " This consumes the `UniqueArc` and returns a regular [`Arc`] that contains the `value` that"] # [doc = " is passed to `into_arc`."] # [doc = ""] # [doc = " Any weak references created before this method is called can now be upgraded to strong"] # [doc = " references."] # [unstable (feature = "unique_rc_arc" , issue = "112566")] # [must_use] pub fn into_arc (this : Self) -> Arc < T , A > { let this = ManuallyDrop :: new (this) ; let alloc : A = unsafe { ptr :: read (& this . alloc) } ; unsafe { (* this . ptr . as_ptr ()) . strong . store (1 , Release) ; Arc :: from_inner_in (this . ptr , alloc) } } }
};
}
