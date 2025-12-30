// Generated macro for impl_84 (impl)
macro_rules! Depcrate_arcimpl_84 {
() => {
// Module: crate::arc
// Provides: {"impl_84"}
// Dependencies: {}
impl < T : ? Sized > UniqueArcUninit < T > { # [doc = " Allocates an ArcInner with layout suitable to contain `for_value` or a clone of it."] fn new (for_value : & T) -> Self { let layout = Layout :: for_value (for_value) ; let ptr = unsafe { Arc :: allocate_for_value (for_value) } ; Self { ptr : NonNull :: new (ptr) . unwrap () , layout_for_value : layout } } # [doc = " Returns the pointer to be written into to initialize the [`Arc`]."] fn data_ptr (& mut self) -> * mut T { let offset = data_offset_align (self . layout_for_value . align ()) ; unsafe { strict :: byte_add (self . ptr . as_ptr () , offset) as * mut T } } # [doc = " Upgrade this into a normal [`Arc`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The data must have been initialized (by writing to [`Self::data_ptr()`])."] unsafe fn into_arc (self) -> Arc < T > { let this = ManuallyDrop :: new (self) ; let ptr = this . ptr . as_ptr () ; unsafe { Arc :: from_ptr (ptr) } } }
};
}
