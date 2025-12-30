// Generated macro for impl_1316 (impl)
macro_rules! Depcrate_rcimpl_1316 {
() => {
// Module: crate::rc
// Provides: {"impl_1316"}
// Dependencies: {}
impl < T : ? Sized , A : Allocator > UniqueRc < T , A > { # [doc = " Converts the `UniqueRc` into a regular [`Rc`]."] # [doc = ""] # [doc = " This consumes the `UniqueRc` and returns a regular [`Rc`] that contains the `value` that"] # [doc = " is passed to `into_rc`."] # [doc = ""] # [doc = " Any weak references created before this method is called can now be upgraded to strong"] # [doc = " references."] # [unstable (feature = "unique_rc_arc" , issue = "112566")] pub fn into_rc (this : Self) -> Rc < T , A > { let mut this = ManuallyDrop :: new (this) ; let alloc : A = unsafe { ptr :: read (& this . alloc) } ; unsafe { this . ptr . as_mut () . strong . set (1) ; Rc :: from_inner_in (this . ptr , alloc) } } }
};
}
