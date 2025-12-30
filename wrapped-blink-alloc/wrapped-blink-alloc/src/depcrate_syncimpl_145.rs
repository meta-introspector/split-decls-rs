// Generated macro for impl_145 (impl)
macro_rules! Depcrate_syncimpl_145 {
() => {
// Module: crate::sync
// Provides: {"impl_145"}
// Dependencies: {}
unsafe impl < A > Allocator for & mut LocalBlinkAlloc < '_ , A > where A : Allocator , { # [inline (always)] fn allocate (& self , layout : Layout) -> Result < NonNull < [u8] > , AllocError > { LocalBlinkAlloc :: allocate (self , layout) } # [inline (always)] fn allocate_zeroed (& self , layout : Layout) -> Result < NonNull < [u8] > , AllocError > { LocalBlinkAlloc :: allocate_zeroed (self , layout) } # [inline (always)] unsafe fn shrink (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout ,) -> Result < NonNull < [u8] > , AllocError > { LocalBlinkAlloc :: resize (self , ptr , old_layout , new_layout) } # [inline (always)] unsafe fn grow (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout ,) -> Result < NonNull < [u8] > , AllocError > { LocalBlinkAlloc :: resize (self , ptr , old_layout , new_layout) } # [inline (always)] unsafe fn deallocate (& self , ptr : NonNull < u8 > , layout : Layout) { LocalBlinkAlloc :: deallocate (self , ptr , layout . size ()) } }
};
}
