// Generated macro for impl_549 (impl)
macro_rules! Depcrate_dtablesimpl_549 {
() => {
// Module: crate::dtables
// Provides: {"impl_549"}
// Dependencies: {}
impl < T > DescriptorTablePointer < T > { pub fn new (tbl : & T) -> Self { let len = size_of :: < T > () - 1 ; assert ! (len < 0x10000) ; DescriptorTablePointer { base : tbl as * const T , limit : len as u16 , } } pub fn new_from_slice (slice : & [T]) -> Self { let len = slice . len () * size_of :: < T > () - 1 ; assert ! (len < 0x10000) ; DescriptorTablePointer { base : slice . as_ptr () , limit : len as u16 , } } }
};
}
