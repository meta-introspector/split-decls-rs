// Generated macro for impl_337 (impl)
macro_rules! Depcrateimpl_337 {
() => {
// Module: crate
// Provides: {"impl_337"}
// Dependencies: {}
# [cfg (any (feature = "allocator_api" , feature = "allocator-api2"))] unsafe impl < 'a , const MIN_ALIGN : usize > Allocator for & 'a Bump < MIN_ALIGN > { # [inline] fn allocate (& self , layout : Layout) -> Result < NonNull < [u8] > , AllocError > { self . try_alloc_layout (layout) . map (| p | unsafe { NonNull :: new_unchecked (ptr :: slice_from_raw_parts_mut (p . as_ptr () , layout . size ())) }) . map_err (| _ | AllocError) } # [inline] unsafe fn deallocate (& self , ptr : NonNull < u8 > , layout : Layout) { Bump :: < MIN_ALIGN > :: dealloc (self , ptr , layout) } # [inline] unsafe fn shrink (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout ,) -> Result < NonNull < [u8] > , AllocError > { Bump :: < MIN_ALIGN > :: shrink (self , ptr , old_layout , new_layout) . map (| p | unsafe { NonNull :: new_unchecked (ptr :: slice_from_raw_parts_mut (p . as_ptr () , new_layout . size ())) }) . map_err (| _ | AllocError) } # [inline] unsafe fn grow (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout ,) -> Result < NonNull < [u8] > , AllocError > { Bump :: < MIN_ALIGN > :: grow (self , ptr , old_layout , new_layout) . map (| p | unsafe { NonNull :: new_unchecked (ptr :: slice_from_raw_parts_mut (p . as_ptr () , new_layout . size ())) }) . map_err (| _ | AllocError) } # [inline] unsafe fn grow_zeroed (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout ,) -> Result < NonNull < [u8] > , AllocError > { let mut ptr = self . grow (ptr , old_layout , new_layout) ? ; ptr . as_mut () [old_layout . size () ..] . fill (0) ; Ok (ptr) } }
};
}
