// Generated macro for impl_117 (impl)
macro_rules! Depcrate_ptrimpl_117 {
() => {
// Module: crate::ptr
// Provides: {"impl_117"}
// Dependencies: {}
impl < T > OwnedPtr < T > { pub (crate) fn new (value : T) -> Self { Self :: from_boxed (Box :: new (value)) } pub (crate) fn from_boxed (boxed : Box < T >) -> Self { Self { ptr : unsafe { NonNull :: new_unchecked (Box :: into_raw (boxed)) } , } } # [doc = " Convert the pointer to another type"] pub (crate) fn cast < U > (self) -> OwnedPtr < U > { OwnedPtr { ptr : self . ptr . cast () , } } # [doc = " Context the pointer into a Box"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Dropping the Box will deallocate a layout of `T` and run the destructor of `T`."] # [doc = ""] # [doc = " A cast pointer must therefore be cast back to the original type before calling this method."] pub (crate) unsafe fn into_box (self) -> Box < T > { unsafe { Box :: from_raw (self . ptr . as_ptr ()) } } pub (crate) const fn as_ref (& self) -> RefPtr < '_ , T > { RefPtr { ptr : self . ptr , _marker : PhantomData , } } pub (crate) fn as_mut (& mut self) -> MutPtr < '_ , T > { MutPtr { ptr : self . ptr , _marker : PhantomData , } } }
};
}
