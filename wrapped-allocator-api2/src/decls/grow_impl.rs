macro_rules! deps {
    () => {
        AllocError!();
    };
}

macro_rules! grow_impl {
    () => {
        deps!();
        # [inline (always)] unsafe fn grow_impl (ptr : NonNull < u8 > , old_layout : Layout , new_layout : Layout , zeroed : bool ,) -> Result < NonNull < [u8] > , AllocError > { debug_assert ! (new_layout . size () >= old_layout . size () , "`new_layout.size()` must be greater than or equal to `old_layout.size()`") ; match old_layout . size () { 0 => alloc_impl (new_layout , zeroed) , old_size if old_layout . align () == new_layout . align () => unsafe { let new_size = new_layout . size () ; assume (new_size >= old_layout . size ()) ; let raw_ptr = System . realloc (ptr . as_ptr () , old_layout , new_size) ; let ptr = NonNull :: new (raw_ptr) . ok_or (AllocError) ? ; if zeroed { raw_ptr . add (old_size) . write_bytes (0 , new_size - old_size) ; } Ok (NonNull :: new_unchecked (core :: ptr :: slice_from_raw_parts_mut (ptr . as_ptr () , new_size ,))) } , old_size => unsafe { let new_ptr = alloc_impl (new_layout , zeroed) ? ; core :: ptr :: copy_nonoverlapping (ptr . as_ptr () , new_ptr . as_ptr () . cast () , old_size) ; System . deallocate (ptr , old_layout) ; Ok (new_ptr) } , } }
    };
}

grow_impl!();