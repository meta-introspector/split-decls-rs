// Generated macro for impl_78 (impl)
macro_rules! Depcrate_arrayvecimpl_78 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_78"}
// Dependencies: {}
impl < T , const CAP : usize > ArrayVecImpl for ArrayVec < T , CAP > { type Item = T ; const CAPACITY : usize = CAP ; fn len (& self) -> usize { self . len () } unsafe fn set_len (& mut self , length : usize) { debug_assert ! (length <= CAP) ; self . len = length as LenUint ; } fn as_ptr (& self) -> * const Self :: Item { self . xs . as_ptr () as _ } fn as_mut_ptr (& mut self) -> * mut Self :: Item { self . xs . as_mut_ptr () as _ } }
};
}
