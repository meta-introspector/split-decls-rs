// Generated macro for impl_1322 (impl)
macro_rules! Depcrate_rcimpl_1322 {
() => {
// Module: crate::rc
// Provides: {"impl_1322"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : ? Sized , A : Allocator > UniqueRcUninit < T , A > { # [doc = " Allocates a RcInner with layout suitable to contain `for_value` or a clone of it."] fn new (for_value : & T , alloc : A) -> UniqueRcUninit < T , A > { let layout = Layout :: for_value (for_value) ; let ptr = unsafe { Rc :: allocate_for_layout (layout , | layout_for_rc_inner | alloc . allocate (layout_for_rc_inner) , | mem | mem . with_metadata_of (ptr :: from_ref (for_value) as * const RcInner < T >) ,) } ; Self { ptr : NonNull :: new (ptr) . unwrap () , layout_for_value : layout , alloc : Some (alloc) } } # [doc = " Returns the pointer to be written into to initialize the [`Rc`]."] fn data_ptr (& mut self) -> * mut T { let offset = data_offset_align (self . layout_for_value . align ()) ; unsafe { self . ptr . as_ptr () . byte_add (offset) as * mut T } } # [doc = " Upgrade this into a normal [`Rc`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The data must have been initialized (by writing to [`Self::data_ptr()`])."] unsafe fn into_rc (self) -> Rc < T , A > { let mut this = ManuallyDrop :: new (self) ; let ptr = this . ptr ; let alloc = this . alloc . take () . unwrap () ; unsafe { Rc :: from_ptr_in (ptr . as_ptr () , alloc) } } }
};
}
