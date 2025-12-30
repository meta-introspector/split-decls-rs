// Generated macro for impl_69 (impl)
macro_rules! Depcrate_atomicimpl_69 {
() => {
// Module: crate::atomic
// Provides: {"impl_69"}
// Dependencies: {}
impl < T : ? Sized + Pointable > Pointer < T > for Shared < '_ , T > { # [inline] fn into_ptr (self) -> * mut () { self . data } # [inline] unsafe fn from_ptr (data : * mut ()) -> Self { Shared { data , _marker : PhantomData , } } }
};
}
