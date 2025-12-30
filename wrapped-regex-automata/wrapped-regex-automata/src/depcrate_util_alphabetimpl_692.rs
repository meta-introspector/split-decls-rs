// Generated macro for impl_692 (impl)
macro_rules! Depcrate_util_alphabetimpl_692 {
() => {
// Module: crate::util::alphabet
// Provides: {"impl_692"}
// Dependencies: {}
impl < 'a > Iterator for ByteClassElementRanges < 'a > { type Item = (Unit , Unit) ; fn next (& mut self) -> Option < (Unit , Unit) > { loop { let element = match self . elements . next () { None => return self . range . take () , Some (element) => element , } ; match self . range . take () { None => { self . range = Some ((element , element)) ; } Some ((start , end)) => { if end . as_usize () + 1 != element . as_usize () || element . is_eoi () { self . range = Some ((element , element)) ; return Some ((start , end)) ; } self . range = Some ((start , element)) ; } } } } }
};
}
