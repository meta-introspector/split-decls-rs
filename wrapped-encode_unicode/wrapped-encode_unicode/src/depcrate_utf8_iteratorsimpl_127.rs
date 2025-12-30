// Generated macro for impl_127 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_127 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_127"}
// Dependencies: {}
impl < U : Borrow < Utf8Char > , I : Iterator < Item = U > > Iterator for Utf8CharSplitter < U , I > { type Item = u8 ; fn next (& mut self) -> Option < Self :: Item > { if self . prev == 0 { self . inner . next () . map (| u8c | { let array = u8c . borrow () . to_array () . 0 ; self . prev = u32 :: from_le_bytes (array) >> 8 ; array [0] }) } else { let next = self . prev as u8 ; self . prev >>= 8 ; Some (next) } } fn size_hint (& self) -> (usize , Option < usize >) { let (min , max) = self . inner . size_hint () ; let add = 4 - (self . prev . leading_zeros () / 8) as usize ; (min . wrapping_add (add) , max . map (| max | max . wrapping_mul (4) . wrapping_add (add))) } }
};
}
