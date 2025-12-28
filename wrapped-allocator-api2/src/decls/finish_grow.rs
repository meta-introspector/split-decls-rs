macro_rules! deps {
    () => {
        AllocError!();
        TryReserveError!();
        Allocator!();
    };
}

macro_rules! finish_grow {
    () => {
        deps!();
        # [inline (always)] fn finish_grow < A > (new_layout : Result < Layout , LayoutError > , current_memory : Option < (NonNull < u8 > , Layout) > , alloc : & mut A ,) -> Result < NonNull < [u8] > , TryReserveError > where A : Allocator , { let new_layout = new_layout . map_err (| _ | CapacityOverflow) ? ; alloc_guard (new_layout . size ()) ? ; let memory = if let Some ((ptr , old_layout)) = current_memory { debug_assert_eq ! (old_layout . align () , new_layout . align ()) ; unsafe { assume (old_layout . align () == new_layout . align ()) ; alloc . grow (ptr , old_layout , new_layout) } } else { alloc . allocate (new_layout) } ; memory . map_err (| _ | { AllocError { layout : new_layout , non_exhaustive : () , } . into () }) }
    };
}

finish_grow!()