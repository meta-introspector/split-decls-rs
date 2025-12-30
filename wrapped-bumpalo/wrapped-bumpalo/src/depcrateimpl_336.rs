// Generated macro for impl_336 (impl)
macro_rules! Depcrateimpl_336 {
() => {
// Module: crate
// Provides: {"impl_336"}
// Dependencies: {}
unsafe impl < 'a , const MIN_ALIGN : usize > alloc :: Alloc for & 'a Bump < MIN_ALIGN > { # [inline (always)] unsafe fn alloc (& mut self , layout : Layout) -> Result < NonNull < u8 > , AllocErr > { self . try_alloc_layout (layout) } # [inline] unsafe fn dealloc (& mut self , ptr : NonNull < u8 > , layout : Layout) { Bump :: < MIN_ALIGN > :: dealloc (self , ptr , layout) ; } # [inline] unsafe fn realloc (& mut self , ptr : NonNull < u8 > , layout : Layout , new_size : usize ,) -> Result < NonNull < u8 > , AllocErr > { let old_size = layout . size () ; if old_size == 0 { return self . try_alloc_layout (layout) ; } let new_layout = layout_from_size_align (new_size , layout . align ()) ? ; if new_size <= old_size { self . shrink (ptr , layout , new_layout) } else { self . grow (ptr , layout , new_layout) } } }
};
}
