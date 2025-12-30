// Generated macro for impl_46 (impl)
macro_rules! Depcrate_bitmapimpl_46 {
() => {
// Module: crate::bitmap
// Provides: {"impl_46"}
// Dependencies: {}
impl < 'a , const SIZE : usize > DoubleEndedIterator for Iter < 'a , SIZE > where BitsImpl < { SIZE } > : Bits , { fn next_back (& mut self) -> Option < Self :: Item > { let result ; match self . tail { None => { result = None ; } Some (index) => { if index >= SIZE { result = self . data . last_index () ; } else { result = self . data . prev_index (index) ; } } } if let Some (index) = result { if let Some (head) = self . head { if head > index { self . head = Some (SIZE + 1) ; self . tail = None ; return None ; } } self . tail = Some (index) ; } else { self . tail = None ; } result } }
};
}
