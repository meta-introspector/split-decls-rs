// Generated macro for impl_273 (impl)
macro_rules! Depcrate_allocimpl_273 {
() => {
// Module: crate::alloc
// Provides: {"impl_273"}
// Dependencies: {}
impl UnstableLayoutMethods for Layout { fn padding_needed_for (& self , align : usize) -> usize { let len = self . size () ; let len_rounded_up = len . wrapping_add (align) . wrapping_sub (1) & ! align . wrapping_sub (1) ; len_rounded_up . wrapping_sub (len) } fn repeat (& self , n : usize) -> Result < (Layout , usize) , LayoutErr > { let padded_size = self . size () . checked_add (self . padding_needed_for (self . align ())) . ok_or_else (new_layout_err) ? ; let alloc_size = padded_size . checked_mul (n) . ok_or_else (new_layout_err) ? ; unsafe { Ok ((Layout :: from_size_align_unchecked (alloc_size , self . align ()) , padded_size ,)) } } fn array < T > (n : usize) -> Result < Layout , LayoutErr > { Layout :: new :: < T > () . repeat (n) . map (| (k , offs) | { debug_assert ! (offs == mem :: size_of ::< T > ()) ; k }) } }
};
}
