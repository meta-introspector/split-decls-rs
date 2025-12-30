// Generated macro for impl_1645 (impl)
macro_rules! Depcrate_syncimpl_1645 {
() => {
// Module: crate::sync
// Provides: {"impl_1645"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : ? Sized , A : Allocator > UniqueArcUninit < T , A > { # [doc = " Allocates an ArcInner with layout suitable to contain `for_value` or a clone of it."] fn new (for_value : & T , alloc : A) -> UniqueArcUninit < T , A > { let layout = Layout :: for_value (for_value) ; let ptr = unsafe { Arc :: allocate_for_layout (layout , | layout_for_arcinner | alloc . allocate (layout_for_arcinner) , | mem | mem . with_metadata_of (ptr :: from_ref (for_value) as * const ArcInner < T >) ,) } ; Self { ptr : NonNull :: new (ptr) . unwrap () , layout_for_value : layout , alloc : Some (alloc) } } # [doc = " Returns the pointer to be written into to initialize the [`Arc`]."] fn data_ptr (& mut self) -> * mut T { let offset = data_offset_align (self . layout_for_value . align ()) ; unsafe { self . ptr . as_ptr () . byte_add (offset) as * mut T } } # [doc = " Upgrade this into a normal [`Arc`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The data must have been initialized (by writing to [`Self::data_ptr()`])."] unsafe fn into_arc (self) -> Arc < T , A > { let mut this = ManuallyDrop :: new (self) ; let ptr = this . ptr . as_ptr () ; let alloc = this . alloc . take () . unwrap () ; unsafe { Arc :: from_ptr_in (ptr , alloc) } } }
};
}
