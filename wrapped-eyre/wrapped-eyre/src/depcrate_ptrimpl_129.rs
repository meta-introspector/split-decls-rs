// Generated macro for impl_129 (impl)
macro_rules! Depcrate_ptrimpl_129 {
() => {
// Module: crate::ptr
// Provides: {"impl_129"}
// Dependencies: {}
impl < 'a , T : ? Sized > MutPtr < 'a , T > { # [doc = " Convert the pointer to another type"] pub (crate) fn cast < U > (self) -> MutPtr < 'a , U > { MutPtr { ptr : self . ptr . cast () , _marker : PhantomData , } } # [doc = " Returns a mutable reference to the owned value with the lifetime decoupled from self"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See: [`NonNull::as_mut`]"] # [inline] pub (crate) unsafe fn into_mut (mut self) -> & 'a mut T { unsafe { self . ptr . as_mut () } } }
};
}
