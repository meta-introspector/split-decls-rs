// Generated macro for impl_358 (impl)
macro_rules! Depcrate_util_alphabetimpl_358 {
() => {
// Module: crate::util::alphabet
// Provides: {"impl_358"}
// Dependencies: {}
impl < 'a > Iterator for ByteClassElementRanges < 'a > { type Item = (u8 , u8) ; fn next (& mut self) -> Option < (u8 , u8) > { loop { let element = match self . elements . next () { None => return self . range . take () , Some (element) => element , } ; match self . range . take () { None => { self . range = Some ((element , element)) ; } Some ((start , end)) => { if usize :: from (end) + 1 != usize :: from (element) { self . range = Some ((element , element)) ; return Some ((start , end)) ; } self . range = Some ((start , element)) ; } } } } }
};
}
