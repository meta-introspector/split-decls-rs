// Generated macro for impl_125 (impl)
macro_rules! Depcrate_localimpl_125 {
() => {
// Module: crate::local
// Provides: {"impl_125"}
// Dependencies: {}
unsafe impl < A > Allocator for & mut BlinkAlloc < A > where A : Allocator , { # [inline (always)] fn allocate (& self , layout : Layout) -> Result < NonNull < [u8] > , AllocError > { BlinkAlloc :: allocate (self , layout) } # [inline (always)] fn allocate_zeroed (& self , layout : Layout) -> Result < NonNull < [u8] > , AllocError > { BlinkAlloc :: allocate_zeroed (self , layout) } # [inline (always)] unsafe fn shrink (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout ,) -> Result < NonNull < [u8] > , AllocError > { BlinkAlloc :: resize (self , ptr , old_layout , new_layout) } # [inline (always)] unsafe fn grow (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout ,) -> Result < NonNull < [u8] > , AllocError > { BlinkAlloc :: resize (self , ptr , old_layout , new_layout) } # [inline (always)] unsafe fn deallocate (& self , ptr : NonNull < u8 > , layout : Layout) { BlinkAlloc :: deallocate (self , ptr , layout . size ()) ; } }
};
}
