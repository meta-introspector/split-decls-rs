// Generated macro for impl_51 (impl)
macro_rules! Depcrate_allocimpl_51 {
() => {
// Module: crate::alloc
// Provides: {"impl_51"}
// Dependencies: {}
impl Global { # [inline] # [cfg_attr (miri , track_caller)] fn alloc_impl (& self , layout : Layout , zeroed : bool) -> Result < NonNull < [u8] > , AllocError > { match layout . size () { 0 => Ok (NonNull :: slice_from_raw_parts (layout . dangling () , 0)) , size => unsafe { let raw_ptr = if zeroed { alloc_zeroed (layout) } else { alloc (layout) } ; let ptr = NonNull :: new (raw_ptr) . ok_or (AllocError) ? ; Ok (NonNull :: slice_from_raw_parts (ptr , size)) } , } } # [inline] # [cfg_attr (miri , track_caller)] unsafe fn grow_impl (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout , zeroed : bool ,) -> Result < NonNull < [u8] > , AllocError > { debug_assert ! (new_layout . size () >= old_layout . size () , "`new_layout.size()` must be greater than or equal to `old_layout.size()`") ; match old_layout . size () { 0 => self . alloc_impl (new_layout , zeroed) , old_size if old_layout . align () == new_layout . align () => unsafe { let new_size = new_layout . size () ; hint :: assert_unchecked (new_size >= old_layout . size ()) ; let raw_ptr = realloc (ptr . as_ptr () , old_layout , new_size) ; let ptr = NonNull :: new (raw_ptr) . ok_or (AllocError) ? ; if zeroed { raw_ptr . add (old_size) . write_bytes (0 , new_size - old_size) ; } Ok (NonNull :: slice_from_raw_parts (ptr , new_size)) } , old_size => unsafe { let new_ptr = self . alloc_impl (new_layout , zeroed) ? ; ptr :: copy_nonoverlapping (ptr . as_ptr () , new_ptr . as_mut_ptr () , old_size) ; self . deallocate (ptr , old_layout) ; Ok (new_ptr) } , } } }
};
}
