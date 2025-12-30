// Generated macro for impl_212 (impl)
macro_rules! Depcrate_utf16_iteratorsimpl_212 {
() => {
// Module: crate::utf16_iterators
// Provides: {"impl_212"}
// Dependencies: {}
impl < U : Borrow < Utf16Char > , I : Iterator < Item = U > > Iterator for Utf16CharSplitter < U , I > { type Item = u16 ; fn next (& mut self) -> Option < Self :: Item > { if self . prev_second == 0 { self . inner . next () . map (| u16c | { let units = u16c . borrow () . to_array () ; self . prev_second = units [1] ; units [0] }) } else { let prev_second = self . prev_second ; self . prev_second = 0 ; Some (prev_second) } } fn size_hint (& self) -> (usize , Option < usize >) { let (min , max) = self . inner . size_hint () ; let add = if self . prev_second == 0 { 0 } else { 1 } ; (min . wrapping_add (add) , max . map (| max | max . wrapping_mul (2) . wrapping_add (add))) } }
};
}
