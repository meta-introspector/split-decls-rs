macro_rules! deps {
    () => {
        Allocator!();
        AllocError!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        unsafe impl < A > Allocator for & A where A : Allocator + ? Sized , { # [inline (always)] fn allocate (& self , layout : Layout) -> Result < NonNull < [u8] > , AllocError > { (* * self) . allocate (layout) } # [inline (always)] fn allocate_zeroed (& self , layout : Layout) -> Result < NonNull < [u8] > , AllocError > { (* * self) . allocate_zeroed (layout) } # [inline (always)] unsafe fn deallocate (& self , ptr : NonNull < u8 > , layout : Layout) { unsafe { (* * self) . deallocate (ptr , layout) } } # [inline (always)] unsafe fn grow (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout ,) -> Result < NonNull < [u8] > , AllocError > { unsafe { (* * self) . grow (ptr , old_layout , new_layout) } } # [inline (always)] unsafe fn grow_zeroed (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout ,) -> Result < NonNull < [u8] > , AllocError > { unsafe { (* * self) . grow_zeroed (ptr , old_layout , new_layout) } } # [inline (always)] unsafe fn shrink (& self , ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout ,) -> Result < NonNull < [u8] > , AllocError > { unsafe { (* * self) . shrink (ptr , old_layout , new_layout) } } }
    };
}

impl_13!()