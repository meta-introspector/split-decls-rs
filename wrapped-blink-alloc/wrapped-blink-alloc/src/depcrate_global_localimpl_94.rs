// Generated macro for impl_94 (impl)
macro_rules! Depcrate_global_localimpl_94 {
() => {
// Module: crate::global::local
// Provides: {"impl_94"}
// Dependencies: {}
impl < A : Allocator > State < A > { # [inline (always)] fn allocate (& self , layout : Layout) -> Result < NonNull < [u8] > , AllocError > { match self . enabled { true => self . blink . allocate (layout) , false => { cold () ; self . blink . inner () . allocate (layout) } } } # [inline (always)] fn allocate_zeroed (& self , layout : Layout) -> Result < NonNull < [u8] > , AllocError > { match self . enabled { true => self . blink . allocate_zeroed (layout) , false => { cold () ; self . blink . inner () . allocate_zeroed (layout) } } } # [inline (always)] unsafe fn resize (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout ,) -> Result < NonNull < [u8] > , AllocError > { match self . enabled { true => self . blink . resize (ptr , old_layout , new_layout) , false => { cold () ; if old_layout . size () >= new_layout . size () { self . blink . inner () . grow (ptr , old_layout , new_layout) } else { self . blink . inner () . shrink (ptr , old_layout , new_layout) } } } } # [inline (always)] unsafe fn deallocate (& self , ptr : NonNull < u8 > , layout : Layout) { match self . enabled { true => self . blink . deallocate (ptr , layout . size ()) , false => { cold () ; self . blink . inner () . deallocate (ptr , layout) } } } }
};
}
