// Generated macro for impl_138 (impl)
macro_rules! Depcrate_syncimpl_138 {
() => {
// Module: crate::sync
// Provides: {"impl_138"}
// Dependencies: {}
unsafe impl < A > Allocator for SyncBlinkAlloc < A > where A : Allocator , { # [inline (always)] fn allocate (& self , layout : Layout) -> Result < NonNull < [u8] > , AllocError > { SyncBlinkAlloc :: allocate (self , layout) } # [inline (always)] unsafe fn shrink (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout ,) -> Result < NonNull < [u8] > , AllocError > { SyncBlinkAlloc :: resize (self , ptr , old_layout , new_layout) } # [inline (always)] unsafe fn grow (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout ,) -> Result < NonNull < [u8] > , AllocError > { SyncBlinkAlloc :: resize (self , ptr , old_layout , new_layout) } # [inline (always)] unsafe fn deallocate (& self , ptr : NonNull < u8 > , layout : Layout) { SyncBlinkAlloc :: deallocate (self , ptr , layout . size ()) ; } }
};
}
