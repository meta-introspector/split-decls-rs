// Generated macro for impl_123 (impl)
macro_rules! Depcrate_ptrimpl_123 {
() => {
// Module: crate::ptr
// Provides: {"impl_123"}
// Dependencies: {}
impl < 'a , T : ? Sized > RefPtr < 'a , T > { pub (crate) fn new (ptr : & 'a T) -> Self { Self { ptr : NonNull :: from (ptr) , _marker : PhantomData , } } # [doc = " Convert the pointer to another type"] pub (crate) fn cast < U > (self) -> RefPtr < 'a , U > { RefPtr { ptr : self . ptr . cast () , _marker : PhantomData , } } # [doc = " Returns a shared reference to the owned value"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See: [`NonNull::as_ref`]"] # [inline] pub (crate) unsafe fn as_ref (& self) -> & 'a T { unsafe { self . ptr . as_ref () } } }
};
}
