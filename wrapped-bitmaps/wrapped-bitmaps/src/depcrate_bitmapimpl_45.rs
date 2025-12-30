// Generated macro for impl_45 (impl)
macro_rules! Depcrate_bitmapimpl_45 {
() => {
// Module: crate::bitmap
// Provides: {"impl_45"}
// Dependencies: {}
impl < 'a , const SIZE : usize > Iterator for Iter < 'a , SIZE > where BitsImpl < { SIZE } > : Bits , { type Item = usize ; fn next (& mut self) -> Option < Self :: Item > { let result ; match self . head { None => { result = self . data . first_index () ; } Some (index) => { if index >= SIZE { result = None } else { result = self . data . next_index (index) ; } } } if let Some (index) = result { if let Some (tail) = self . tail { if tail < index { self . head = Some (SIZE + 1) ; self . tail = None ; return None ; } } else { self . head = Some (SIZE + 1) ; return None ; } self . head = Some (index) ; } else { self . head = Some (SIZE + 1) ; } result } }
};
}
