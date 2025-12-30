// Generated macro for finish_grow (function)
macro_rules! Depcrate_raw_vecfinish_grow {
() => {
// Module: crate::raw_vec
// Provides: {"finish_grow"}
// Dependencies: {}
# [cold] fn finish_grow < A > (new_layout : Layout , current_memory : Option < (NonNull < u8 > , Layout) > , alloc : & mut A ,) -> Result < NonNull < [u8] > , TryReserveError > where A : Allocator , { let memory = if let Some ((ptr , old_layout)) = current_memory { debug_assert_eq ! (old_layout . align () , new_layout . align ()) ; unsafe { hint :: assert_unchecked (old_layout . align () == new_layout . align ()) ; alloc . grow (ptr , old_layout , new_layout) } } else { alloc . allocate (new_layout) } ; memory . map_err (| _ | AllocError { layout : new_layout , non_exhaustive : () } . into ()) }
};
}
