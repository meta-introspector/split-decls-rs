// Generated macro for impl_113 (impl)
macro_rules! Depcrate_comparisonimpl_113 {
() => {
// Module: crate::comparison
// Provides: {"impl_113"}
// Dependencies: {}
impl SortKeyLevel { fn len (& self) -> usize { self . buf . len () } fn is_empty (& self) -> bool { self . buf . is_empty () } fn append_byte (& mut self , x : u8) { self . buf . push (x) ; } fn append_weight_16 (& mut self , w : u16) { debug_assert_ne ! (w , 0) ; let b0 = (w >> 8) as u8 ; let b1 = w as u8 ; self . append_byte (b0) ; if b1 != 0 { self . append_byte (b1) ; } } fn append_reverse_weight_16 (& mut self , w : u16) { debug_assert_ne ! (w , 0) ; let b0 = (w >> 8) as u8 ; let b1 = w as u8 ; if b1 != 0 { self . append_byte (b1) ; } self . append_byte (b0) ; } fn append_weight_32 (& mut self , w : u32) { debug_assert_ne ! (w , 0) ; let b0 = (w >> 24) as u8 ; let b1 = (w >> 16) as u8 ; let b2 = (w >> 8) as u8 ; let b3 = w as u8 ; self . append_byte (b0) ; if b1 != 0 { self . append_byte (b1) ; if b2 != 0 { self . append_byte (b2) ; if b3 != 0 { self . append_byte (b3) ; } } } } }
};
}
